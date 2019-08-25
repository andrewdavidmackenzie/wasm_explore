run: add_function/target/wasm32-unknown-unknown/debug/add_function.wasm
	@cargo run

clean:
	@cargo clean
	@rm -rf add_function/target
	@rm -f add_function/add_function.wat

add_function/target/wasm32-unknown-unknown/debug/add_function.wasm: add_function/src/add_function.rs
	@cd add_function; cargo build
	@wasm2wat add_function/target/wasm32-unknown-unknown/debug/add_function.wasm -o add_function/add_function.wat