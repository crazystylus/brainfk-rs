use std::path::PathBuf;

use crate::Language;
use clap::{Parser, Subcommand};

pub trait Run {
    fn run(&self) -> Result<(), std::io::Error>;
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
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
pub enum Target {
    /// Allows running in a browser
    Browser,
    /// Allows running natively
    Wasi,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum Backend {
    Cranelift,
    LLVM,
    Singlepass,
}

impl Run for Cli {
    fn run(&self) -> Result<(), std::io::Error> {
        self.command.run()
    }
}

impl Run for Command {
    fn run(&self) -> Result<(), std::io::Error> {
        match self {
            Command::Run {
                input_file,
                backend,
            } => {
                let input_path = PathBuf::from(input_file);
                let mut brainfk = Language::new(&input_path, false);
                brainfk.parse()?;
                brainfk.generate_wasm(&Target::Wasi)?; // Only WASI can run natively
                brainfk.validate().unwrap();
                brainfk.optimize();
                brainfk.compile_wasmu(&backend).unwrap();
                brainfk.run();
                Ok(())
            }
            Command::GenerateWasm {
                input_file,
                output_file,
                target,
            } => {
                let input_path = PathBuf::from(input_file);
                let mut brainfk = Language::new(&input_path, false);
                brainfk.parse()?;
                brainfk.generate_wasm(&target)?;
                brainfk.validate().unwrap();
                brainfk.optimize();
                brainfk.write_wasm(&output_file)?;
                println!("✔ Successfully generated wasm.");
                Ok(())
            }
            Command::CompileWasmu {
                input_file,
                output_file,
                backend,
            } => {
                let input_path = PathBuf::from(input_file);
                let mut brainfk = Language::new(&input_path, false);
                brainfk.parse()?;
                brainfk.generate_wasm(&Target::Wasi)?;
                brainfk.validate().unwrap();
                brainfk.optimize();
                brainfk.compile_wasmu(&backend).unwrap();
                brainfk.write_wasmu(output_file).unwrap();
                println!("✔ Compiled successfully to wasmu.");
                println!("Compiled file can be executed using wasmer-headless.");
                Ok(())
            }
            Command::CompileBinary {
                input_file,
                output_file,
                backend,
            } => {
                let input_path = PathBuf::from(input_file);
                let mut brainfk = Language::new(&input_path, false);
                brainfk.parse()?;
                brainfk.generate_wasm(&Target::Wasi)?; // Only WASI can generate a binary
                brainfk.validate().unwrap();
                brainfk.optimize();
                brainfk.compile_binary(&output_file, &backend).unwrap();
                Ok(())
            }
        }
    }
}
