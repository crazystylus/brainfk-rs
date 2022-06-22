# Brainfk -> WASM
## Introduction
This repo contains a brainfk compiler written in Rust closely tied to wasm.

### Features
- Generate wasm from brainfk code
- Compile and run brainfk code
- Compile brainfk to wasmu (wasmer module serial format) which can run with wasmer-headless
- Supports following backends
  - LLVM (uses LLVM 12)
  - Cranelift
  - Singlepass

### Usage
```
# JIT Compile & Execute brainfk code
$ brainfk-rs run tests/files/hello.bf --backend cranelift
Hello World!

$ brainfk-rs generate-wasm tests/files/hello.bf hello.wasm --target wasi
✔ Successfully generated wasm.

$ brainfk-rs compile-wasmu tests/files/hello.bf hello.wasmu --backend cranelift
✔ Compiled successfully to wasmu.
Compiled file can be executed using wasmer-headless.

# Running in wasmer-headless
$ ./wasmer-headless run hello.wasmu
Hello World!
```

### Install Wasmer Runtime
Follow this link to install wasmer-runtime
https://docs.wasmer.io/ecosystem/wasmer/getting-started

### TODO
- [ ] Memory access validation and growth
- [x] Integrate with wasmer and support run & compile
- [ ] Add testing
- [ ] Add support for JS/Browser
