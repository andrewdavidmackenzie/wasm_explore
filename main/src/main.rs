extern crate implementation;
extern crate memory_units;
#[macro_use]
extern crate serde_json;
extern crate wrapper;

use std::fs::File;
use std::io::Read;

use add::Add;
use implementation::Implementation;
use wrapper::wasm_simulator::WasmSimulator;

mod wasm;

fn main() {
    println!("Running in {}", std::env::current_dir().unwrap().display());
    let inputs = vec!(vec!(json!(1)), vec!(json!(2)));

    // Run the function usin gnative-native rust
    let implementation = &Add{} as &dyn Implementation;
    // Call the function
    let (result, run_again) = implementation.run(inputs.clone());
    println!("\nNative\n=========\nResult = {}, run_again = {}", result.unwrap(), run_again);

    // Create a Simulated wasm executor
    let implementation = &WasmSimulator::new(&Add{}) as &dyn Implementation;
    // Call the function via the wasm simulator
    let (result, run_again) = implementation.run(inputs.clone());
    println!("\nSimulator\n=========\nResult = {}, run_again = {}", result.unwrap(), run_again);

    // load the hand crafter version
    let mut filename = "main/add_memory.wasm";
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

    // Load the version compiled to wasm from rust
    filename = "add/add.wasm";
    buffer = Vec::new();
    file = File::open(filename).unwrap();
    file.read_to_end(&mut buffer).unwrap();
    println!("\nRust compiled to WASM\n=========\nLoading wasm module from '{}'", filename);
    let rust_wasm_executor = wasm::load(buffer, "add".into());

    // Run the function using the Implementation trait function 'run'
    let (result, run_again) = rust_wasm_executor.run(inputs);
    match result {
        Some(res) => println!("Result = {}, run_again = {}", res, run_again),
        _ => {}
    }
}
