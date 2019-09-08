all: build/add_function.wat run

run: build/target/wasm32-unknown-unknown/release/build.wasm
	@cargo run

clean:
	@cd build; cargo clean
	@cargo clean
	@rm -f build/add_function.wat

build/add_function.wat: build/target/wasm32-unknown-unknown/release/build.wasm
	wasm2wat $< -o build/add_function.wat

build/target/wasm32-unknown-unknown/release/build.wasm: add_function/src/add_function.rs build/Cargo.toml ./Makefile
	cd build; cargo build --release --lib --target=wasm32-unknown-unknown
	wasm-gc $@

# Other things you might want to play with
# Use wasm-opt: but you have to install the binaryen toolset - not via cargo
# wasm-opt -Oz $@
# wasm-strip $@ - removes custom sections of a wasm
# wasm-snip $@ - requires name section so requires debug build
# wasm-gc → wasm-snip → wasm-gc → wasm-opt.