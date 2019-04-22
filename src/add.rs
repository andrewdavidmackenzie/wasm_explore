use super::implementation::{Implementation, RunAgain};
use serde_json;
use serde_json::Value;

pub struct Add;

impl Implementation for Add {
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, RunAgain) {
        let input_a = inputs.get(0).unwrap().get(0).unwrap();
        let input_b = inputs.get(1).unwrap().get(0).unwrap();

        let value = Some(json!(input_a.as_i64().unwrap() + input_b.as_i64().unwrap()));

        (value, true)
    }
}