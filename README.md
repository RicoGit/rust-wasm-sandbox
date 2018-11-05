
### Compile rust to wasm binary

    cargo  +nightly build --target wasm32-unknown-unknown --release

### Run with [wasm-interp](https://github.com/WebAssembly/wabt#running-wasm-interp)

    wasm-interp target/wasm32-unknown-unknown/release/rust_wasm_sandbox.wasm --run-all-exports
    
### Runs with [Ocaml Wasm interpreter](https://github.com/WebAssembly/spec/tree/master/interpreter)
    
    wasm target/wasm32-unknown-unknown/release/rust_wasm_sandbox.wasm -t -e '(invoke "test")'
    
### Runs with [Asmble Vm](https://github.com/cretz/asmble)
    
    asmble invoke -res -in target/wasm32-unknown-unknown/release/rust_wasm_sandbox.wasm test -log info -defmaxmempages 32