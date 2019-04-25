#[macro_use]
extern crate serde_json;

mod add;
mod implementation;
mod wrapper;
mod wasm;

use implementation::Implementation;
use crate::wrapper::Wrapper;
use add::Add;
use std::fs::File;
use std::io::Read;

fn main() {
    let inputs = vec!(vec!(json!(1)), vec!(json!(2)));
    let implementation = &Wrapper{implementation: &Add{}} as &Implementation;
    let (result, run_again) = implementation.run(inputs.clone());
    println!("Result = {}, run_again = {}", result.unwrap(), run_again);

    let mut buffer = Vec::new();
    let mut file = File::open("add.wasm").unwrap();
    file.read_to_end(&mut buffer).unwrap();
    let wasm_executor = wasm::load("add", buffer);
    let (result, run_again) = wasm_executor.run(inputs);
    println!("Result = {}, run_again = {}", result.unwrap(), run_again);
}
