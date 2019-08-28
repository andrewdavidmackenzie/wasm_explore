extern crate core;
extern crate implementation;
#[macro_use]
extern crate serde_json;
extern crate wasm_impl_derive;

use implementation::Implementation;
use serde_json::Value;
use wasm_impl_derive::WasmImpl;

#[derive(WasmImpl)]
pub struct Add;

impl Implementation for Add {
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, bool) {
        let input_a = inputs.get(0).unwrap().get(0).unwrap();
        let input_b = inputs.get(1).unwrap().get(0).unwrap();

        let value = Some(json!(input_a.as_i64().unwrap() + input_b.as_i64().unwrap()));

        (value, true)
    }
}