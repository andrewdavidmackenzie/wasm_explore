use serde_json::Value;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::marker::{Sync, Send};

pub type RunAgain = bool;

pub trait Implementation : RefUnwindSafe + UnwindSafe + Sync + Send {
    // An implementation can be run, with an array of inputs and returns a value (or null) and a
    // bool indicating if it should be ran again
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, RunAgain);
}

pub trait WasmImplementation {
    fn run(&self, input_data: Vec<u8>) -> (Vec<u8>, RunAgain);
}