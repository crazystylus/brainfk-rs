use brainfk_rs::cmd::{Backend, Target};
use brainfk_rs::Language;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub fn bench_helper(input_file: &PathBuf) -> Language {
    let mut brainfk = Language::new(input_file, true);
    brainfk.parse().unwrap();
    brainfk.generate_wasm(&Target::Wasi).unwrap();
    brainfk.validate().unwrap();
    brainfk.optimize();
    brainfk.compile_wasmu(&Backend::LLVM).unwrap();
    brainfk
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // List all .bf files in files
    let mut tests_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    tests_dir.push("benches");
    tests_dir.push("code");
    let bf_files: Vec<PathBuf> = fs::read_dir(tests_dir)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .filter_map(|ele| match ele {
            Ok(x) => Some(x),
            Err(_) => None,
        })
        .filter(|x| x.extension().unwrap() == "bf")
        .collect();

    let mut group = c.benchmark_group("WASM_Bench");
    group.sample_size(15);
    group.warm_up_time(Duration::from_secs(60));

    for bf_file in bf_files {
        let bf_module = bench_helper(&bf_file);
        group.bench_with_input(
            BenchmarkId::new("Mandelbrot", bf_file.file_name().unwrap().to_str().unwrap()),
            &bf_module,
            |b, _bf_module| b.iter(|| bf_module.run()),
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
