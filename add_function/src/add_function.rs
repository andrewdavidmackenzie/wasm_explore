#[macro_use]
extern crate serde_json;

use serde_json::Value;

pub struct Add;

#[no_mangle]
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}

impl Add {
    // TODO add_function the macro here that wraps this function
#[no_mangle]
    extern "C" fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, bool) {
        let input_a = inputs.get(0).unwrap().get(0).unwrap();
        let input_b = inputs.get(1).unwrap().get(0).unwrap();

        let value = Some(json!(input_a.as_i64().unwrap() + input_b.as_i64().unwrap()));

        (value, true)
    }

#[no_mangle]
    pub extern "C" fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}