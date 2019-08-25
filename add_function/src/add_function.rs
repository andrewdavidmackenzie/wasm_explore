extern crate implementation;
#[macro_use]
extern crate serde_json;

use serde_json::Value;

use implementation::Implementation;

pub struct Add;

#[no_mangle]
pub extern "C" fn run_wasm(a: u32, b: u32) -> u32 {
    let inputs = vec!(vec!(json!(a)), vec!(json!(b)));
    let adder = Add{};
    let (value, _bool) = adder.run(inputs);
    value.unwrap().as_u64().unwrap() as u32
}

impl Implementation for Add {
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, bool) {
        let input_a = inputs.get(0).unwrap().get(0).unwrap();
        let input_b = inputs.get(1).unwrap().get(0).unwrap();

        let value = Some(json!(input_a.as_i64().unwrap() + input_b.as_i64().unwrap()));

        (value, true)
    }
}

// TODO add_function the macro here that wraps this function
impl Add {
}