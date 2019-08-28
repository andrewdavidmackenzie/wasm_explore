use wasmi::{ImportsBuilder, Module, ModuleInstance};

use wrapper::wasm::WasmExecutor;

/*
    load a Wasm module from the specified Url.
*/
pub fn load(content: Vec<u8>) -> WasmExecutor {
    let module = Module::from_buffer(content).unwrap();

    let module_ref = ModuleInstance::new(&module, &ImportsBuilder::default())
        .unwrap()
        .assert_no_start();

    let memory = module_ref.export_by_name("memory")
        .expect("`memory` export not found")
        .as_memory()
        .expect("export name `memory` is not of memory type")
        .to_owned();

    WasmExecutor::new(module_ref, memory)
}