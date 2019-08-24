all: hand_crafted/add.wasm hand_crafted/add_memory.wasm wasm run

clean:
	cargo clean

hand_crafted/%.wasm: hand_crafted/%.wat
	wat2wasm $<

wasm: rust_add
	# Compile rust to wasm using specific Cargo.toml for the purpose
	cp rust_add/Cargo_wasm.toml rust_add/Cargo.toml
	cd rust_add; cargo build --manifest-path=Cargo.toml --target=wasm32-unknown-unknown
	# Copy compiled wasm file into the root folder
	cp rust_add/target/wasm32-unknown-unknown/debug/add.wasm rust_add/add.wasm
	#restore native build Cargo.toml for main build to use
	cp rust_add/Cargo_native.toml rust_add/Cargo.toml
	# decompile so we can look at the code
	wasm2wat rust_add/target/wasm32-unknown-unknown/debug/add.wasm -o rust_add/add.wat

run:
	cargo run