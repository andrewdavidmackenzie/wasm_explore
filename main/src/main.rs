extern crate implementation;
extern crate memory_units;
#[macro_use]
extern crate serde_json;
extern crate wrapper;

use serde_json::Value;
use std::fs::File;
use std::io::Read;

use implementation::Implementation;

mod wasm;

fn run_wasm(filename: &str, inputs: &Vec<Vec<Value>>) {
    println!("Loading wasm module from '{}'", filename);
    let mut buffer = Vec::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_end(&mut buffer).unwrap();
    let wasm_executor = wasm::load(buffer);

    // Run the function using the Implementation trait function 'run'
    let (result, run_again) = wasm_executor.run(inputs.clone());

    match result {
        Some(res) => println!("Result = {}, run_again = {}", res, run_again),
        _ => {}
    }
}

fn main() {
    println!("Running in {}", std::env::current_dir().unwrap().display());
    let inputs = vec!(vec!(json!(1)), vec!(json!(2)));

    println!("\nHandcrafted WASM\n===========");
    run_wasm("hand_crafted/add.wasm", &inputs);
    println!("\nRust compiled to WASM\n===========");
    run_wasm("add_function/target/wasm32-unknown-unknown/debug/add_function.wasm", &inputs);
}
