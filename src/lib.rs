pub mod cmd;
mod compiler;

use crate::cmd::{Backend, Target};
use std::collections::HashSet;
use std::env::temp_dir;
use std::fs;
use std::io;
use std::path::PathBuf;

use uuid::Uuid;
use wasm_encoder::EntityType;
use wasm_encoder::ExportSection;
use wasm_encoder::{
    CodeSection, Function, FunctionSection, ImportSection, Instruction, MemorySection, MemoryType,
    Module, TypeSection, ValType,
};
use wasm_pack::PBAR;
use wasm_pack::{cache, wasm_opt};
use wasmer::CompileError;
use wasmer::Module as WasmerModule;
use wasmer::{Cranelift, Instance, Singlepass, Store, Universal, LLVM};
use wasmer_wasi::{Pipe, Stdin, Stdout, WasiState};

pub struct Language<'a> {
    /// This contains the characterset from brainfk language
    pub char_set: HashSet<char>,
    /// Character by language symbols
    pub code: Vec<char>,
    /// Generated bytecode
    pub wasm_bytes: Vec<u8>,
    /// Wasmer module
    pub module: Option<wasmer::Module>,
    /// Suppress I/O streams (useful for benching)
    pub suppress_io: bool,
    /// Input file name
    pub input_file: &'a PathBuf,
}

impl<'a> Language<'a> {
    pub fn new(input_file: &'a PathBuf, suppress_io: bool) -> Self {
        Self {
            char_set: HashSet::from(['<', '>', '+', '-', '.', ',', '[', ']']),
            code: Vec::new(),
            wasm_bytes: Vec::new(),
            module: None,
            suppress_io,
            input_file,
        }
    }

    /// Open and parse the code
    pub fn parse(&mut self) -> Result<(), io::Error> {
        let path = std::path::Path::new(&self.input_file);
        let content = fs::read_to_string(path)?;
        self.code = content
            .chars()
            .filter(|x| self.char_set.contains(x))
            .collect();
        Ok(())
    }

    /// Generate WASM bytecode
    pub fn generate_wasm(&mut self, target: &Target) -> Result<(), io::Error> {
        // Create a new module
        let mut module = Module::new();
        // Type section for void function
        let mut types = TypeSection::new();
        // Types for main function
        types.function([], []);
        match target {
            Target::Browser => {
                // Types for JS functions
                todo!();
            }
            Target::Wasi => {
                // Types for fd_read and fd_write
                types.function(
                    vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                    vec![ValType::I32],
                );
                types.function(
                    vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                    vec![ValType::I32],
                );
            }
        }
        module.section(&types);
        let mut imports = ImportSection::new();
        match target {
            Target::Browser => {
                // Import JS functions
                todo!();
            }
            Target::Wasi => {
                // Import WASI functions
                imports.import("wasi_unstable", "fd_read", EntityType::Function(1));
                imports.import("wasi_unstable", "fd_write", EntityType::Function(2));
            }
        }
        module.section(&imports);

        let mut functions = FunctionSection::new();
        let type_index = 0;
        functions.function(type_index);
        module.section(&functions);

        let mut memories = MemorySection::new();
        // Memory for Input Stream
        memories.memory(MemoryType {
            minimum: 3, // 30,000 elements roughly 2 pages
            maximum: None,
            memory64: false,
            shared: false,
        });

        module.section(&memories);

        let mut exports = ExportSection::new();
        exports.export("memory", wasm_encoder::ExportKind::Memory, 0);
        exports.export("_start", wasm_encoder::ExportKind::Func, 2);
        module.section(&exports);

        // let start = match target {
        //     Target::Browser => StartSection { function_index: 0 },
        //     Target::Wasi => StartSection { function_index: 2 },
        // };
        // module.section(&start);

        let mut codes = CodeSection::new();
        // Local Declaration
        // Local 0 : I32 TAPE Pointer
        // Local 1 : I32 I/O Buffer pointer
        // Local 2 : I32 I/O Buffer max-size
        // Local 3 : I32 I/O Vector start
        // ...
        let locals = vec![(4, ValType::I32)];
        let mut f = Function::new(locals);

        // <-- Linear Memory Model -->
        // ---------------------------
        // | I/O Buffer | I/O Vectors |  TAPE    |
        // 0-----------516-----------1024------4096|
        // TODO: Check if memory pointer invalid
        // TODO: Tape expansion
        f.instruction(&Instruction::I32Const(1024));
        f.instruction(&Instruction::LocalSet(0));
        f.instruction(&Instruction::I32Const(0));
        f.instruction(&Instruction::LocalSet(1));
        f.instruction(&Instruction::I32Const(1000));
        f.instruction(&Instruction::LocalSet(2));
        f.instruction(&Instruction::I32Const(1004));
        f.instruction(&Instruction::LocalSet(3));

        // Symbol matching
        let mut fcount: u32 = 2;
        for symb in &self.code {
            match symb {
                '<' => compiler::less_than(&mut f),
                '>' => compiler::greater_than(&mut f),
                '+' => compiler::plus(&mut f),
                '-' => compiler::minus(&mut f),
                ',' => compiler::comma(&mut f, target),
                '.' => compiler::dot(&mut f, target),
                '[' => {
                    fcount += 1;
                    compiler::sq_start(&mut f, fcount);
                }
                ']' => compiler::sq_end(&mut f),
                _ => {
                    f.instruction(&Instruction::Nop);
                }
            }
        }

        // Flush Stdout
        compiler::flush_stdout(&mut f, target);
        // Mark program end
        f.instruction(&Instruction::End);

        codes.function(&f);
        module.section(&codes);

        self.wasm_bytes = module.finish();
        Ok(())
    }

    /// Validate generated WASM bytecode
    pub fn validate(&self) -> Result<wasmparser::types::Types, wasmparser::BinaryReaderError> {
        wasmparser::validate(&self.wasm_bytes)
    }

    pub fn optimize(&mut self) {
        // Generate tmpfir and files
        let tmp_dir = temp_dir();

        let mut unopt_file = tmp_dir.clone();
        unopt_file.set_file_name(Uuid::new_v4().simple().to_string());
        unopt_file.set_extension(".wasm");

        fs::write(&unopt_file, &self.wasm_bytes).unwrap();

        // Set wasm-bindgen STDOUt logging to quiet
        PBAR.set_quiet(true);

        // Download and run wasm-opt
        wasm_opt::run(
            &cache::get_wasm_pack_cache().unwrap(),
            tmp_dir.as_path(),
            &["--flatten --precompute --optimize-instructions --local-cse".to_string()],
            //&["-O4".to_string()],
            true,
        )
        .unwrap();

        // Update wasm-bytes
        self.wasm_bytes = fs::read(&unopt_file).unwrap();
    }

    /// Write generated WASM bytecode to file
    pub fn write_wasm(&self, output_file: &str) -> Result<(), io::Error> {
        fs::write(output_file, &self.wasm_bytes)
    }

    /// Generate wasmu
    pub fn compile_wasmu(&mut self, backend: &Backend) -> Result<(), CompileError> {
        let engine = match backend {
            &Backend::LLVM => Universal::new(LLVM::default()).engine(),
            &Backend::Cranelift => Universal::new(Cranelift::default()).engine(),
            &Backend::Singlepass => Universal::new(Singlepass::default()).engine(),
        };
        let store = Store::new(&engine);
        self.module = Some(WasmerModule::new(&store, &self.wasm_bytes)?);
        Ok(())
    }

    /// Write generated WASM bytecode to file
    pub fn write_wasmu(&self, output_file: &str) -> Result<(), wasmer::SerializeError> {
        self.module.as_ref().unwrap().serialize_to_file(output_file)
    }

    /// TODO: Compile to a binary using wasmer
    pub fn compile_binary(&self, _output_file: &str, _backend: &Backend) -> Result<(), io::Error> {
        todo!();
    }

    /// Run bf-code
    pub fn run(&self) {
        let module = self.module.as_ref().unwrap();

        // Suppresses IO during benching
        let mut wasi_env = if self.suppress_io {
            WasiState::new("brainfk")
                .stdin(Box::new(Pipe::new()))
                .stdout(Box::new(Pipe::new()))
                .finalize()
                .unwrap()
        } else {
            WasiState::new("brainfk")
                .stdin(Box::new(Stdin))
                .stdout(Box::new(Stdout))
                .finalize()
                .unwrap()
        };
        let import_object = wasi_env.import_object(&module).unwrap();
        let instance = Instance::new(&module, &import_object).unwrap();
        let start = instance.exports.get_function("_start").unwrap();
        start.call(&[]).unwrap();
    }
}