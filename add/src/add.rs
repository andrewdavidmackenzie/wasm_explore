extern crate implementation;

use implementation::implementation::{Implementation, WasmImplementation};
use implementation::simulator;

#[macro_use]
extern crate serde_json;

use serde_json::Value;

pub struct Add;

impl Add {
    pub fn run(_size: i32) {}
}

// TODO generate this from a macro to wrap native code?
// TODO that is only generated when the target is wasm?
// TODO renames the real implementation or adds a new wrapper one?
impl WasmImplementation for Add {
    fn run_wasm(&self, size: u32) -> usize {
        // get input parameters from wasm module memory
        let input_slice = simulator::get_module_memory(size);
        let inputs: Vec<Vec<Value>> = serde_json::from_slice(input_slice).unwrap();

        // run the actual implementation
        let (result, run_again) = self.run(inputs);

        // get the results and store them back to wasm module memory
        let result_data = serde_json::to_vec(&(result, run_again)).unwrap();
        let result_data_size = result_data.len();
        simulator::set_module_memory(result_data.as_slice());

        // return the size of the output in module memory
        result_data_size
    }
}

impl Implementation for Add {
    // TODO add the macro here that wraps this function
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, bool) {
        let input_a = inputs.get(0).unwrap().get(0).unwrap();
        let input_b = inputs.get(1).unwrap().get(0).unwrap();

        let value = Some(json!(input_a.as_i64().unwrap() + input_b.as_i64().unwrap()));

        (value, true)
    }
}