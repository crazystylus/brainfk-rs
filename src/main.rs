mod compiler;
mod lang;
use clap::{Parser, Subcommand};
use lang::Language;
use std::io;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate wasm file
    GenerateWasm {
        /// .b/.BF file for generating wasm
        #[clap(value_parser)]
        input_file: String,
        /// Output file name
        #[clap(value_parser)]
        output_file: String,
        #[clap(long, value_parser = clap::value_parser!(Target))]
        target: Target,
    },
    /// Compile to wasmu
    CompileWasmu {
        /// .b/.BF file for compiling
        #[clap(value_parser)]
        input_file: String,
        /// Output file name
        #[clap(value_parser)]
        output_file: String,
        /// Backend to use for compiling
        #[clap(long, value_parser = clap::value_parser!(Backend))]
        backend: Backend,
    },
    /// Compile to binary
    CompileBinary {
        /// .b/.BF file for compiling
        #[clap(value_parser)]
        input_file: String,
        /// Output file name
        #[clap(value_parser)]
        output_file: String,
        /// Backend to use for compiling
        #[clap(long, value_parser = clap::value_parser!(Backend))]
        backend: Backend,
    },
    /// Run
    Run {
        /// .b/.BF file to run
        #[clap(value_parser)]
        input_file: String,
        /// Backend to use for compiling
        #[clap(long, value_parser = clap::value_parser!(Backend))]
        backend: Backend,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
enum Target {
    /// Allows running in a browser
    Browser,
    /// Allows running natively
    Wasi,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
enum Backend {
    Cranelift,
    LLVM,
    Singlepass,
}

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();
    let mut brainfk = Language::new();
    match &cli.command {
        Command::Run {
            input_file,
            backend,
        } => {
            brainfk.parse(input_file)?;
            brainfk.generate_wasm(&Target::Wasi)?; // Only WASI can run natively
            brainfk.validate().unwrap();
            brainfk.run(backend);
        }
        Command::GenerateWasm {
            input_file,
            output_file,
            target,
        } => {
            brainfk.parse(input_file)?;
            brainfk.generate_wasm(target)?;
            brainfk.validate().unwrap();
            brainfk.write_wasm(output_file)?;
            println!("✔ Successfully generated wasm.");
        }
        Command::CompileWasmu {
            input_file,
            output_file,
            backend,
        } => {
            brainfk.parse(input_file)?;
            brainfk.generate_wasm(&Target::Wasi)?;
            brainfk.validate().unwrap();
            brainfk.compile_wasmu(output_file, backend).unwrap();
            println!("✔ Compiled successfully to wasmu.");
            println!("Compiled file can be executed using wasmer-headless.");
        }
        Command::CompileBinary {
            input_file,
            output_file,
            backend,
        } => {
            brainfk.parse(input_file)?;
            brainfk.generate_wasm(&Target::Wasi)?; // Only WASI can generate a binary
            brainfk.validate().unwrap();
            brainfk.compile_binary(output_file, backend).unwrap();
        }
    }
    Ok(())
}
