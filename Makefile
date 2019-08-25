all: wasm wat run

wasm:
	@cd add_function; cargo build

wat:
	@# decompile so we can look at the code
	@wasm2wat add_function/target/wasm32-unknown-unknown/debug/add_function.wasm -o add_function/add_function.wat

run:
	@cargo run
