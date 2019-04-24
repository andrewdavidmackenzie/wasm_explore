use super::implementation::Implementation;
use serde_json::Value;
use crate::implementation::WasmImplementation;
use std::slice;

pub struct Wrapper<'a> {
    pub implementation: &'a WasmImplementation
}

static mut MEMORY: [u8; 1000] = [0; 1000];

pub fn get_module_memory(size: u32) -> &'static [u8] {
    unsafe {
        let ptr = &MEMORY as *const _;
        slice::from_raw_parts(ptr, size as usize)
    }
}

pub fn set_module_memory(contents: &[u8]) {
    unsafe {
//        MEMORY.to_vec().write_all(contents).unwrap();
        let dest_ptr = MEMORY.as_mut_ptr() as * mut u8;
        let src_ptr = contents.as_ptr() as * const u8;
        std::ptr::copy(src_ptr, dest_ptr, contents.len())
    };
}

impl<'a> Implementation for Wrapper<'a> {
    fn run(&self, inputs: Vec<Vec<Value>>) -> (Option<Value>, bool) {
        let input_data = serde_json::to_vec(&inputs).unwrap();
        set_module_memory(input_data.as_slice());

        let result_date_size = self.implementation.run_wasm(input_data.len() as u32) as u32;

        let result_data = get_module_memory(result_date_size);
        let (result, run_again) = serde_json::from_slice(result_data).unwrap();
        (result, run_again)
    }
}