all: wasm run

clean:
	cargo clean

%.wat : %.wasm
	wasm2wat $<

wasm:
	cargo build --manifest-path=add_function/Cargo.toml
	# decompile so we can look at the code
	wasm2wat add_function/target/wasm32-unknown-unknown/debug/add_function.wasm -o add_function/add_function.wat

run:
	cargo run