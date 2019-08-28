extern crate core;
extern crate implementation;
#[macro_use]
extern crate serde_json;

use serde_json::Value;

use implementation::Implementation;

pub struct Add;
use std::mem;
use std::os::raw::c_void;
use std::ptr::copy;

/*
    Allocate a chunk of memory of `size` bytes in wasm module
*/
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn run_wasm(input_data_ptr: *mut c_void, input_data_length: i32) -> i32 {
    let input_data: Vec<u8> = unsafe {
        Vec::from_raw_parts(input_data_ptr as *mut u8,
                              input_data_length as usize, input_data_length as usize)
    };

    let inputs = serde_json::from_slice(&input_data).unwrap();
    let adder = Add {};
    let result = adder.run(inputs);

    let return_data = serde_json::to_vec(&result).unwrap();

    unsafe { copy(return_data.as_ptr(), input_data_ptr as *mut u8, return_data.len()); }

    return_data.len() as i32
}

impl Implementation for Add {
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, bool) {
        let input_a = inputs.get(0).unwrap().get(0).unwrap();
        let input_b = inputs.get(1).unwrap().get(0).unwrap();

        let value = Some(json!(input_a.as_i64().unwrap() + input_b.as_i64().unwrap()));

        (value, true)
    }
}