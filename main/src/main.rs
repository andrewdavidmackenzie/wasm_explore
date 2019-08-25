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

fn hand_crafted(inputs: &Vec<Vec<Value>>) {
    // load the hand crafted version
    let filename = "hand_crafted/add.wasm";
    let mut buffer = Vec::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_end(&mut buffer).unwrap();
    println!("\nHandcrafted WASM\n====\nLoading wasm module from '{}'", filename);
    let wasm_executor = wasm::load(buffer, "add".into());

    // Run the function using the Implementation trait function 'run'
    let (result, run_again) = wasm_executor.run(inputs.clone());

    match result {
        Some(res) => println!("Result = {}, run_again = {}", res, run_again),
        _ => {}
    }
}

fn rust_compiled_wasm(inputs: &Vec<Vec<Value>>) {
    // Load the version compiled to wasm from rust
    let filename = "add_function/target/wasm32-unknown-unknown/debug/add_function.wasm";
    let mut buffer = Vec::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_end(&mut buffer).unwrap();
    println!("\nRust compiled to WASM\n=========\nLoading wasm module from '{}'", filename);
    let rust_wasm_executor = wasm::load(buffer, "add".into());

    // Run the function using the Implementation trait function 'run'
    let (result, run_again) = rust_wasm_executor.run(inputs.clone());
    match result {
        Some(res) => println!("Result = {}, run_again = {}", res, run_again),
        _ => {}
    }
}

fn main() {
    println!("Running in {}", std::env::current_dir().unwrap().display());
    let inputs = vec!(vec!(json!(1)), vec!(json!(2)));

    hand_crafted(&inputs);
    rust_compiled_wasm(&inputs);
}
