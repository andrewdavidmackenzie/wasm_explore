

use std::sync::Mutex;

use serde_json::Value;

use implementation::{Implementation, RunAgain};
use wasmi::{MemoryInstance, ModuleRef, NopExternals, RuntimeValue};
use wasmi::memory_units::*;

pub struct WasmExecutor {
    module: Mutex<ModuleRef>
}

impl WasmExecutor {
    pub fn new(module_ref: ModuleRef) -> Self {
        WasmExecutor {
            module: Mutex::new(module_ref)
        }
    }
}

impl Implementation for WasmExecutor {
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, RunAgain) {
        // setup module memory with the serialized version of inputs Vec<Vec<Value>>
        let linear_memory = MemoryInstance::alloc(Pages(1), None).unwrap();
        let input_data = serde_json::to_vec(&inputs).unwrap();
        linear_memory.set(0, input_data.as_slice()).unwrap();
        println!("Allocated Memory and set to serialized input values");
        // let input_value = input_date.len();
        //let mut args = Vec::<RuntimeValue>::new();
        // args.push(RuntimeValue::from(input_value));

        // TODO passing the actual values as two i32 - not Vec<Vec<Value>>
        let input_a = inputs.get(0).unwrap().get(0).unwrap().as_u64().unwrap() as u32;
        let input_b = inputs.get(1).unwrap().get(0).unwrap().as_u64().unwrap() as u32;

        let mut args = Vec::<RuntimeValue>::new();
        args.push(RuntimeValue::from(input_a));
        args.push(RuntimeValue::from(input_b));

        // call the wasm implementation function (by name)
        let module_ref = self.module.lock().unwrap();
        let result = module_ref.invoke_export("add",
                                              &args, &mut NopExternals);

        match result {
            Ok(value) => {
                match value {
                    Some(RuntimeValue::I32(sum)) => {
                        // read the module memory and reconstruct the return tuple
                        /*
                        let ret_value: i32 = 0; // TODO when wasm returns the size of the serialized return tuple
                        let result_data = linear_memory.get(0, result_date_size).unwrap().as_slice();
                        let (result, run_again) = serde_json::from_slice(result_data).unwrap();
                        */

                        (Some(json!(sum)), true)
                    }
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
