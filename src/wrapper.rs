use super::implementation::Implementation;
use serde_json::Value;
use crate::implementation::WasmImplementation;

pub struct Wrapper<'a> {
    pub implementation: &'a WasmImplementation
}

impl<'a> Implementation for Wrapper<'a> {
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, bool) {
        let input_data = serde_json::to_vec(&inputs).unwrap();

        let (result_data, run_again) = self.implementation.run_wasm(input_data);

        let result = serde_json::from_slice(&result_data).unwrap();

        (result, run_again)
    }
}