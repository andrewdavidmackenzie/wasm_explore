all: build/add_function.wat run

run: build/target/wasm32-unknown-unknown/debug/build.wasm
	@cargo run

clean:
	@cd build; cargo clean
	@cargo clean
	@rm -f build/add_function.wat

build/add_function.wat: build/target/wasm32-unknown-unknown/debug/build.wasm
	wasm2wat $< -o build/add_function.wat

build/target/wasm32-unknown-unknown/debug/build.wasm: add_function/src/add_function.rs build/Cargo.toml ./Makefile
	cd build; cargo build --target=wasm32-unknown-unknown