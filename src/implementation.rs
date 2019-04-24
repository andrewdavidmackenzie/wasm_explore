use serde_json::Value;

pub type RunAgain = bool;

pub trait Implementation {
    // An implementation can be run, with an array of inputs and returns a value (or null) and a
    // bool indicating if it should be ran again
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, RunAgain);
}

pub trait WasmImplementation {
    fn run_wasm(&self, size: u32) -> usize;
}