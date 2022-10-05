# Brainfk -> WASM
## Introduction
This repo contains a brainfk compiler written in Rust closely tied to wasm.

### Features
- Generate wasm from brainfk code
- Compile and run brainfk code
- Compile brainfk to wasmu (wasmer module serial format) which can run with wasmer-headless
- Supports following backends
  - LLVM (uses LLVM 14)
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

## Benchmarks

| Command | Mean [s] | Min [s] | Max [s] |
|:---|---:|---:|---:|
| `mandelbrot-extreme.bf` | 72.278 | 72.273 | 72.282 |
| `mandelbrot-titannic.bf` | 13.979 | 13.975 | 13.982 |
| `mandelbrot-huge.bf` | 2.9724 | 2.9729 | 2.9733 |
| `mandelbrot.bf` | 0.56329 | 0.56318  | 0.56340 |
| `mandelbrot-tiny.bf` | 0.14279 | 0.14277  | 0.14281 |

### Benchmark Environment
```
CPU:Ryzen 5 5600X (12) @ 3.700GHz
Memory: 16GB DDR4 3400 MT/s
Linux: Pop!_OS 22.04 LTS x86_64
Kernel: 5.19.0-76051900-generic
Rust: rustc 1.64.0 (a55dd71d5 2022-09-19)
Toolchain Target: stable-x86_64-unknown-linux-gnu
```

### Performing a Benchmark
Criterion-rs is used for benchmarking and report is generated in target dir.
```
cargo build --release --bench
cargo bench
```

## TODO
- [ ] Runtime memory check and growth
- [x] Integrate with wasmer and support run & compile
- [x] Add testing
- [x] Benchmarks
- [x] Add support for JS/Browser
- Optimizations
  + [x] Stdout Buffering
  + [x] Utilize wasm-opt
