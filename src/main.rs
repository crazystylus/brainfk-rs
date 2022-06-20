mod compiler;
mod lang;
use clap;
use clap::Parser;
use lang::Language;
use std::{fs, io};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// File to compile
    #[clap(value_parser)]
    input_file: String,
    /// Output file name without extension
    #[clap(value_parser)]
    output_file: String,
    /// Target for runtime
    #[clap(long, value_parser = clap::value_parser!(Target))]
    target: Target,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
enum Target {
    Browser,
    Wasi,
}

impl Args {
    pub fn run(&self) -> Result<(), io::Error> {
        let path = std::path::Path::new(&self.input_file);
        let content = fs::read_to_string(path)?;
        let brainfk = Language::new();
        let code: Vec<char> = content
            .chars()
            .filter(|x| brainfk.char_set.contains(x))
            .collect();
        let wasm_bytes = brainfk.code_gen(&code, &self.target)?;
        brainfk.validate(&wasm_bytes, &self.output_file)
    }
}

fn main() -> Result<(), io::Error> {
    Args::parse().run()?;
    Ok(())
}
