## Benchmarking
### Summary
- `hyperfine` is used for bencmarking
- `brainfk-rs` only compiles code to wasmu
- Compiled code is then execute by `wasmer-headless`

### Steps
1. Compile `brainfk-rs`: `cargo build --release`
2. Compiling all code files to `.wasmu`:
```shell
mkdir -p /tmp/bench/compiled
for i in $(ls code)
do
../target/release/brainfk-rs compile-wasmu "code/${i}" --backend llvm "/tmp/bench/compiled/${i}.wasmu"
done
```
3. Download and extract `wasmer-runtime`
```shell
pushd /tmp/bench/
wget https://github.com/wasmerio/wasmer/releases/download/2.3.0/wasmer-linux-amd64.tar.gz
tar -xvf wasmer-linux-amd64.tar.gz
popd
```
4. Execute hyperfine for benchmark results
```shell
pushd /tmp/bench/
hyperfine --warmup 3 -L file mandelbrot-extreme.bf.wasmu,mandelbrot-huge.bf.wasmu,mandelbrot-tiny.bf.wasmu,mandelbrot-titannic.bf.wasmu,mandelbrot.bf.wasmu './bin/wasmer-headless run compiled/{file}' --export-markdown bench.md
popd
```
