all: build/add_function.wat run

run: add_function/target/wasm32-unknown-unknown/debug/add_function.wasm
	@RUST_BACKTRACE=1 cargo run

clean:
	@cd add_function; cargo clean
	@cargo clean
	@rm -f add_function/add_function.wat

add_function/build.wat: add_function/target/wasm32-unknown-unknown/debug/add_function.wasm
	wasm2wat $< -o build/add_function.wat

add_function/target/wasm32-unknown-unknown/debug/add_function.wasm: add_function/src/add_function.rs
	cd add_function; cargo build --target=wasm32-unknown-unknown