use std::sync::Mutex;
use std::cmp::max;
use serde_json::Value;
use wasmi::{ModuleRef, MemoryRef, NopExternals, RuntimeValue};

use implementation::{Implementation, RunAgain};

const MAX_RESULT_SIZE: i32 = 1024;

pub struct WasmExecutor {
    module: Mutex<ModuleRef>,
    memory: Mutex<MemoryRef>
}

impl WasmExecutor {
    pub fn new(module_ref: ModuleRef, memory: MemoryRef) -> Self {
        WasmExecutor {
            module: Mutex::new(module_ref),
            memory: Mutex::new(memory)
        }
    }
}

/*
    Allocate memory for array of bytes inside the wasm module and copy the array of bytes into it
*/
fn send_byte_array(instance: &ModuleRef, memory: &MemoryRef, bytes: &[u8]) -> u32 {
    let alloc_size = max(bytes.len() as i32, MAX_RESULT_SIZE);
    let result = instance
        .invoke_export("alloc", &[RuntimeValue::I32(alloc_size)],
                       &mut NopExternals);

    match result.unwrap().unwrap() {
        RuntimeValue::I32(pointer) => {
            memory.set(pointer as u32, bytes).unwrap();
            pointer as u32
        }
        _ => 0 as u32
    }
}

impl Implementation for WasmExecutor {
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, RunAgain) {
        // call the wasm implementation function (by name)
        let module_ref = self.module.lock().unwrap();
        let memory_ref = self.memory.lock().unwrap();

        // setup module memory with the serde serialization of `inputs: Vec<Vec<Value>>`
        let input_data = serde_json::to_vec(&inputs).unwrap();

        // Allocate a string for the input data inside wasm module
        let input_data_wasm_ptr = send_byte_array(&module_ref, &memory_ref, &input_data);

        println!("Running the exported function 'run_wasm'");
        let result = module_ref.invoke_export("run_wasm",
                                              &[RuntimeValue::I32(input_data_wasm_ptr as i32),
                                                  RuntimeValue::I32(input_data.len() as i32),], &mut NopExternals);

        match result {
            Ok(value) => {
                match value.unwrap() {
                    RuntimeValue::I32(result_length) => {
                        if result_length > MAX_RESULT_SIZE {
                            (None, true)
                        } else {
                            let result_data = memory_ref.get(input_data_wasm_ptr, result_length as usize).unwrap();
                            let (result, run_again) = serde_json::from_slice(result_data.as_slice()).unwrap();
                            (result, run_again)
                        }
                    }
                    _ =>  (None, true)
                }
            }
            Err(err) => {
                println!("Error returned by Wasm invoke_export(): {:?}", err);
                (None, true)
            }
        }
    }
}
