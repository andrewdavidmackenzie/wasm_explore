

use std::slice;

use implementation::{Implementation, RunAgain};
use serde_json::Value;

pub struct WasmSimulator<'a> {
    wrapper: Wrapper<'a>
}

struct Wrapper<'a> {
    inner_implementation: &'a dyn Implementation
}

/*
    In real version this code would be injected into the function code and compiled to wasm
    with it, and so would be running in wasm and would read from Wasm module linear memory.

    Here its reading from a simulated linear memory
*/
impl<'a> Wrapper<'a> {
    fn run(&self, size: u32) -> usize {
        // get input parameters from wasm module memory
        let input_slice = WasmSimulator::get_module_memory(size);
        let inputs: Vec<Vec<Value>> = serde_json::from_slice(input_slice).unwrap();

        // run the actual implementation
        let (result, run_again) = self.inner_implementation.run(inputs);

        // get the results and store them back to wasm module memory
        let result_data = serde_json::to_vec(&(result, run_again)).unwrap();
        let result_data_size = result_data.len();
        WasmSimulator::set_module_memory(result_data.as_slice());

        // return the size of the output in module memory
        result_data_size
    }
}

static mut MEMORY: [u8; 1000] = [0; 1000];

impl<'a> WasmSimulator<'a> {
    pub fn new(implementation: &'a dyn Implementation) -> Self {
        WasmSimulator {
            wrapper: Wrapper {inner_implementation: implementation}
        }
    }

    fn get_module_memory(size: u32) -> &'static [u8] {
        unsafe {
            let ptr = &MEMORY as *const _;
            slice::from_raw_parts(ptr, size as usize)
        }
    }

    fn set_module_memory(contents: &[u8]) {
        unsafe {
//        MEMORY.to_vec().write_all(contents).unwrap();
            let dest_ptr = MEMORY.as_mut_ptr() as *mut u8;
            let src_ptr = contents.as_ptr() as *const u8;
            std::ptr::copy(src_ptr, dest_ptr, contents.len())
        };
    }
}

/*
    Called from runtime, this function executes natively.
    It serializes the inputs into linear memory and then calls the inner function (that in wasm
    version would be running in wasm), which deserializes the inputs from linear memory and then
    calls the real function implementation (which would also be running in wasm).
*/
impl Implementation for WasmSimulator<'_> {
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, RunAgain) {
        // Linearize inputs into module memory
        let input_data = serde_json::to_vec(&inputs).unwrap();
        WasmSimulator::set_module_memory(input_data.as_slice());

        // Run the wasm function that wraps the compiled implementation
        let result_date_size = self.wrapper.run(input_data.len() as u32) as u32;

        // delinearize the output of the module memory into return values
        let result_data = WasmSimulator::get_module_memory(result_date_size);
        let (result, run_again) = serde_json::from_slice(result_data).unwrap();
        (result, run_again)
    }
}