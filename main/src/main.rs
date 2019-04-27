#[macro_use]
extern crate serde_json;
extern crate memory_units;

mod wasm;

extern crate implementation;
use implementation::implementation::Implementation;
use implementation::simulator::Wrapper;
use add::Add;
use std::fs::File;
use std::io::Read;

fn main() {
    let inputs = vec!(vec!(json!(1)), vec!(json!(2)));
    let implementation = &Wrapper{implementation: &Add{}} as &Implementation;
    let (result, run_again) = implementation.run(inputs.clone());
    println!("Simulator\nResult = {}, run_again = {}", result.unwrap(), run_again);

    let filename = "add_memory.wasm";
    let mut buffer = Vec::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_end(&mut buffer).unwrap();
    println!("\nWASM\nLoading wasm module from '{}'", filename);
    let wasm_executor = wasm::load(buffer);

    let (result, run_again) = wasm_executor.run(inputs);

    match result {
        Some(res) => println!("Result = {}, run_again = {}", res, run_again),
        _ => {}
    }
}
