all: main/add.wasm main/add_memory.wasm rust_add rust

clean:
	rm *.wasm

%.wasm: %.wat
	wat2wasm $<

rust_add: add/src/add.rs
	cd add; cargo build --target=wasm32-unknown-unknown

rust:
	cd main; cargo run