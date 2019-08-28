# wasm-explore
In this repo I explore (imagine someone groping around on their hands and knees in a totally 
dark room for pieces of a puzzle) how to send arbitrary structure data across the native/wasm
boundary, using serde serialization and deserialization.

I am aware that this could be done more efficently using "C" structures, but this is in fact
for another project where the structures are defined already and where I want to send data
between processes as serialized JSON by serde anyway - so this fits perfectly.

# Clone Repo
```
git clone https://github.com/andrewdavidmackenzie/wasm_explore
```

# Build and Run
`cd` into the folder and run `make`

This first builds the wasm module (in /add_function folder) from rust code there.

Then it builds and runs the program (in /main folder) that runs the wasm module using
WASMI.

```
$ cd wasm_explore
$ make
cd add_function; cargo build
   Compiling ryu v0.2.7
   Compiling serde v1.0.90
   Compiling itoa v0.4.3
   Compiling serde_json v1.0.39
   Compiling implementation v0.0.1 (/Users/andrew/workspace/wasm_explore/implementation)
   Compiling add_function v0.0.1 (/Users/andrew/workspace/wasm_explore/add_function)
    Finished dev [unoptimized + debuginfo] target(s) in 14.30s
# rustc +nightly --crate-type cdylib --target wasm32-unknown-unknown add_function/src/add_function.rs
wasm2wat add_function/target/wasm32-unknown-unknown/debug/add_function.wasm -o add_function/add_function.wat
   Compiling libc v0.2.62
   Compiling ryu v1.0.0
   Compiling serde v1.0.99
   Compiling getrandom v0.1.11
   Compiling byteorder v1.3.2
   Compiling cfg-if v0.1.9
   Compiling lazy_static v1.3.0
   Compiling ppv-lite86 v0.2.5
   Compiling itoa v0.4.4
   Compiling memory_units v0.3.0
   Compiling memory_units v0.4.0
   Compiling c2-chacha v0.2.2
   Compiling parity-wasm v0.31.3
   Compiling rand_core v0.5.0
   Compiling rand_chacha v0.2.1
   Compiling rand v0.7.0
   Compiling wasmi v0.4.2
   Compiling serde_json v1.0.40
   Compiling implementation v0.0.1 (/Users/andrew/workspace/wasm_explore/implementation)
   Compiling add_function v0.0.1 (/Users/andrew/workspace/wasm_explore/add_function)
   Compiling wrapper v0.0.1 (/Users/andrew/workspace/wasm_explore/wrapper)
   Compiling main v0.1.0 (/Users/andrew/workspace/wasm_explore/main)
    Finished dev [unoptimized + debuginfo] target(s) in 22.16s
     Running `target/debug/main`
Running in /Users/andrew/workspace/wasm_explore

Rust compiled to WASM
===========
Loading wasm module from 'add_function/target/wasm32-unknown-unknown/debug/add_function.wasm'
Running the exported function 'run_wasm'
Ran successfully and result as expected
```
