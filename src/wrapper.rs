use super::implementation::Implementation;
use serde_json::Value;
use super::add::Add;
use crate::implementation::WasmImplementation;

const IMPLEMENTATION: &WasmImplementation= &Add{} as &WasmImplementation;

pub struct Wrapper;

impl Implementation for Wrapper {
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, bool) {
        let input_data = serde_json::to_vec(&inputs).unwrap();

        let (result_data, run_again) = IMPLEMENTATION.run(input_data);

        let result = serde_json::from_slice(&result_data).unwrap();

        (result, run_again)
    }
}