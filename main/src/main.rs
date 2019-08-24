extern crate implementation;
extern crate memory_units;
#[macro_use]
extern crate serde_json;
extern crate wrapper;

use serde_json::Value;
use std::fs::File;
use std::io::Read;

use add::Add;
use implementation::Implementation;
use wrapper::wasm_simulator::WasmSimulator;

mod wasm;

fn native_rust(inputs: &Vec<Vec<Value>>) {
    // Run the function using native-native rust
    let implementation = &Add{} as &dyn Implementation;
    // Call the function
    let (result, run_again) = implementation.run(inputs.clone());
    println!("\nNative\n=========\nResult = {}, run_again = {}", result.unwrap(), run_again);
}

fn wasm_simulator(inputs: &Vec<Vec<Value>>) {
    // Create a Simulated wasm executor
    let implementation = &WasmSimulator::new(&Add{}) as &dyn Implementation;
    // Call the function via the wasm simulator
    let (result, run_again) = implementation.run(inputs.clone());
    println!("\nSimulator\n=========\nResult = {}, run_again = {}", result.unwrap(), run_again);
}

fn hand_crafted(inputs: &Vec<Vec<Value>>) {
    // load the hand crafted version
    let filename = "hand_crafted/add.wasm";
    let mut buffer = Vec::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_end(&mut buffer).unwrap();
    println!("\nHandcrafted WASM\n====\nLoading wasm module from '{}'", filename);
    let wasm_executor = wasm::load(buffer, "rust_add".into());

    // Run the function using the Implementation trait function 'run'
    let (result, run_again) = wasm_executor.run(inputs.clone());

    match result {
        Some(res) => println!("Result = {}, run_again = {}", res, run_again),
        _ => {}
    }
}

fn rust_compiled_wasm(inputs: &Vec<Vec<Value>>) {
    // Load the version compiled to wasm from rust
    let filename = "rust_add/add.wasm";
    let mut buffer = Vec::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_end(&mut buffer).unwrap();
    println!("\nRust compiled to WASM\n=========\nLoading wasm module from '{}'", filename);
    let rust_wasm_executor = wasm::load(buffer, "rust_add".into());

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

    native_rust(&inputs);
    wasm_simulator(&inputs);
    hand_crafted(&inputs);
    rust_compiled_wasm(&inputs);
}
