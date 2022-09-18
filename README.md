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

## Benchmarks

| Command | Mean [s] | Min [s] | Max [s] |
|:---|---:|---:|---:|
| `mandelbrot-extreme.bf` | 107.083 ± 0.351 | 106.708 | 107.750 |
| `mandelbrot-titannic.bf` | 20.341 ± 0.067 | 20.242 | 20.415 |
| `mandelbrot-huge.bf` | 4.222 ± 0.057 | 4.109 | 4.288 |
| `mandelbrot.bf` | 0.835 ± 0.005 | 0.828 | 0.844 |
| `mandelbrot-tiny.bf` | 0.204 ± 0.002 | 0.202 | 0.208 |

### Benchmark Environment
```
CPU: i7-1165G7 (8) @ 4.700GHz
Memory: 16GB LPDDR4 4267 MT/s
Linux: Pop!_OS 22.04 LTS x86_64
Kernel: 5.17.15-76051715-generic
Rust: rustc 1.61.0 (fe5b13d68 2022-05-18)
Toolchain Target: stable-x86_64-unknown-linux-gnu
```

### Performing a Benchmark
Criterion-rs is used for benchmarking and report is generated in target dir.
```
cargo build --release --bench
cargo bench
```

## TODO
- [ ] Memory access validation and growth
- [x] Integrate with wasmer and support run & compile
- [x] Add testing
- [x] Benchmarks
- [ ] Add support for JS/Browser
- Optimizations
  + [x] Stdout Buffering
  + [ ] Reducing consecutive `+` and `-` operators
