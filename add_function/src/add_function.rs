extern crate implementation;

use implementation::Implementation;

#[macro_use]
extern crate serde_json;

use serde_json::Value;

pub struct Add;

impl Implementation for Add {
    // TODO add_function the macro here that wraps this function
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, bool) {
        let input_a = inputs.get(0).unwrap().get(0).unwrap();
        let input_b = inputs.get(1).unwrap().get(0).unwrap();

        let value = Some(json!(input_a.as_i64().unwrap() + input_b.as_i64().unwrap()));

        (value, true)
    }
}

impl Add {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}