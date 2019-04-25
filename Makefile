all: add.wasm add_memory.wasm rust

clean:
	rm *.wasm

%.wasm: %.wat
	wat2wasm $<

rust:
	cargo run