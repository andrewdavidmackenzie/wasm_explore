use std::sync::Mutex;

use crate::implementation::Implementation;
use crate::implementation::RunAgain;
use serde_json::Value;
use wasmi::{Module, ModuleRef, ModuleInstance, ImportsBuilder};
//use wasmi::{Module, ModuleRef, ModuleInstance, ImportsBuilder, RuntimeValue, NopExternals};

use serde_json;

pub struct WasmExecutor {
    pub module: Mutex<ModuleRef>,
    function_name: String,
}

impl Implementation for WasmExecutor {
    fn run(&self, _inputs: Vec<Vec<Value>>) -> (Option<Value>, RunAgain) {
        println!("Wasm implementation wrapper for function '{}' called", self.function_name);

        // TODO setup module memory
        // TODO call the wasm implementation function (by name?) and get the result
        // TODO read the module memory and reconstruct the return tuple

        /*
          A wasm module is invoked thus:
            pub fn invoke_export<E: Externals>(&self, func_name: &str, args: &[RuntimeValue],
                                    externals: &mut E) -> Result<Option<RuntimeValue>, Error>

        let res = module.invoke_export(self.function_name, &[RuntimeValue::from(inputs)],
                                  &mut NopExternals).unwrap().unwrap();
        res
        */
        (Some(json!(42)), true)
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
        module: Mutex::new(module_ref.clone()),
        function_name: function_name.to_string(),
    };

    executor
}