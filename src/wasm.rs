use std::sync::Mutex;

use crate::implementation::Implementation;
use crate::implementation::RunAgain;
use serde_json::Value;
use wasmi::{Module, ModuleRef, ModuleInstance, ImportsBuilder, RuntimeValue, NopExternals};

use serde_json;

pub struct WasmExecutor {
    pub module: Mutex<ModuleRef>,
    function_name: String,
}

impl Implementation for WasmExecutor {
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, RunAgain) {
        println!("WasmExecutor for function '{}' called", self.function_name);

        // TODO setup module memory
        // TODO call the wasm implementation function (by name?) and get the result
        // TODO read the module memory and reconstruct the return tuple

        let input_a = inputs.get(0).unwrap().get(0).unwrap().as_u64().unwrap() as u32;
        let input_b = inputs.get(1).unwrap().get(0).unwrap().as_u64().unwrap() as u32;

        let mut args = Vec::<RuntimeValue>::new();
        args.push(RuntimeValue::from(input_a));
        args.push(RuntimeValue::from(input_b));

        let module = self.module.lock().unwrap();
        let result = module.invoke_export(&self.function_name,
                                       &args, &mut NopExternals);

        match result {
            Ok(value) => {
                match value {
                    Some(RuntimeValue::I32(sum)) => (Some(json!(sum)), true),
                    Some(_) => {
                        println!("Got a value of an unexpected type");
                        (None, true)
                    }
                    None => {
                        println!("Failed to get a result from wasm invocation");
                        (None, true)
                    }
                }
            }
            Err(err) => {
                println!("Error returned by Wasm invoke_export(): {:?}", err);
                (None, true)
            }
        }
    }
}

/*
    load a Wasm module from the specified Url.
*/
pub fn load(function_name: &str, content: Vec<u8>) -> WasmExecutor {
    let module = Module::from_buffer(content).unwrap();

    let module_ref = ModuleInstance::new(&module, &ImportsBuilder::default())
        .unwrap()
        .assert_no_start();

    let executor = WasmExecutor {
        module: Mutex::new(module_ref),
        function_name: function_name.to_string(),
    };

    executor
}