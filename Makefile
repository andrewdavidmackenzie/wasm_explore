all: main/add.wasm main/add_memory.wasm wasm run

clean:
	rm *.wasm

%.wasm: %.wat
	wat2wasm $<

wasm: add/src/add.rs
	cp add/Cargo_wasm.toml add/Cargo.toml
	cd add; cargo build --manifest-path=Cargo.toml --target=wasm32-unknown-unknown
	cp add/target/wasm32-unknown-unknown/debug/add.wasm add/add.wasm
	wasm2wat add/target/wasm32-unknown-unknown/debug/add.wasm -o add/add.wat

run:
	cp add/Cargo_native.toml add/Cargo.toml #restore native build Cargo.toml for main build to use
	cd main; cargo run