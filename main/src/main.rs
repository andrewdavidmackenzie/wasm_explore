extern crate implementation;
extern crate memory_units;
extern crate rand;
#[macro_use]
extern crate serde_json;
extern crate wrapper;

use std::fs::File;
use std::io::Read;

use rand::Rng;
use serde_json::Value;

use implementation::Implementation;
use wrapper::wasm::WasmExecutor;

mod loader;

fn run_wasm(wasm_executor: &WasmExecutor, inputs: &Vec<Vec<Value>>, expected: &Value) {
    let (result, run_again) = wasm_executor.run(inputs.clone());

    match result {
        Some(res) => {
            assert_eq!(expected.clone(), res);
            assert!(run_again);
            println!("Ran wasm with input '{:?}' and result '{}' is as expected", inputs, res);
        }
        _ => assert!(false)
    }
}

fn main() {
    let filename = "build/target/wasm32-unknown-unknown/release/build.wasm";
    println!("Loading wasm module from '{}'", filename);
    let mut buffer = Vec::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_end(&mut buffer).unwrap();
    let wasm_executor = loader::load(buffer);

    let mut rng = rand::thread_rng();

    for _ in 0..3 {
        let i1: u32 = rng.gen_range(0, 1000);
        let i2: u32 = rng.gen_range(0, 1000);
        let sum: u32 = i1 + i2;

        let inputs = vec!(vec!(json!(i1)), vec!(json!(i2)));
        let expected = json!(sum);

        run_wasm(&wasm_executor,
                 &inputs, &expected);
    }
}