# Brainfk -> WASM
## Introduction
This repo contains a brainfk compiler written in Rust which compiles code directly to wasm. Later the generated wasm file can be ran using wasmer.
### Install Wasmer Runtime
Follow this link to install wasmer-runtime
https://docs.wasmer.io/ecosystem/wasmer/getting-started

### Example
```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/brainfk-rs`
error: The following required arguments were not provided:
    --target <TARGET>
    <INPUT_FILE>
    <OUTPUT_FILE>

USAGE:
    brainfk-rs --target <TARGET> <INPUT_FILE> <OUTPUT_FILE>

For more information try --help

$ cargo run -- tests/files/hello.f hello.wasm --target wasi
   Compiling brainfk-rs v0.1.0 (/root/Documents/brainfk-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.91s
     Running `target/debug/brainfk-rs tests/files/hello.f abc.wasm --target wasi`

$ wasmer run hello.wasm
Hello World!
```
### Support
- [X] WASI (It uses STDIN and STDOUT)
- [ ] Browser

### TODO
- [ ] Memory access validation and growth
- [ ] Add support for JS/Browser
- [ ] Integrate with wasmer and support run & compile
- [ ] Add testing
