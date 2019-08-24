extern crate implementation;
extern crate wrapper;

use wasmi::{ImportsBuilder, Module, ModuleInstance};

use wrapper::wasm::WasmExecutor;

/*
    load a Wasm module from the specified Url.
*/
pub fn load(content: Vec<u8>, function_name: String) -> WasmExecutor {
    let module = Module::from_buffer(content).unwrap();

    let module_ref = ModuleInstance::new(&module, &ImportsBuilder::default())
        .unwrap()
        .assert_no_start();

    WasmExecutor::new(module_ref, function_name)
}