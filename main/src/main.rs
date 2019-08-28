extern crate implementation;
extern crate memory_units;
#[macro_use]
extern crate serde_json;
extern crate wrapper;
extern crate rand;

use rand::Rng;

use serde_json::Value;
use std::fs::File;
use std::io::Read;

use implementation::Implementation;

mod loader;

fn run_wasm(filename: &str, inputs: &Vec<Vec<Value>>, expected: &Value) {
    println!("Loading wasm module from '{}'", filename);
    let mut buffer = Vec::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_end(&mut buffer).unwrap();
    let wasm_executor = loader::load(buffer);

    // Run the function
    println!("Running function with input '{:?}'", inputs);
    let (result, run_again) = wasm_executor.run(inputs.clone());

    match result {
        Some(res) => {
            assert_eq!(expected.clone(), res);
            assert!(run_again);
            println!("Ran successfully and result '{}' is as expected", res);
        },
        _ => assert!(false)
    }
}

fn main() {
    println!("Running in {}", std::env::current_dir().unwrap().display());
    let mut rng = rand::thread_rng();
    let i1: u32 = rng.gen_range(0, 1000);
    let i2: u32 = rng.gen_range(0, 1000);
    let sum: u32 = i1 + i2;

    let inputs = vec!(vec!(json!(i1)), vec!(json!(i2)));
    let expected = json!(sum);

    println!("\nRust compiled to WASM\n===========");
    run_wasm("add_function/target/wasm32-unknown-unknown/debug/add_function.wasm",
             &inputs, &expected);
}