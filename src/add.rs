use super::implementation::{Implementation, RunAgain, WasmImplementation};
use serde_json;
use serde_json::Value;

pub struct Add;

impl WasmImplementation for Add {
    fn run_wasm(&self, input_data: Vec<u8>) -> (Vec<u8>, RunAgain) {
        let inputs: Vec<Vec<Value>> = serde_json::from_slice(input_data.as_slice()).unwrap();

        let (result, run_again) = self.run(inputs);

        let result_data = serde_json::to_vec(&result).unwrap();

        (result_data, run_again)
    }
}

impl Implementation for Add {
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, bool) {
        let input_a = inputs.get(0).unwrap().get(0).unwrap();
        let input_b = inputs.get(1).unwrap().get(0).unwrap();

        let value = Some(json!(input_a.as_i64().unwrap() + input_b.as_i64().unwrap()));

        (value, true)
    }
}