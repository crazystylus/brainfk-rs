use crate::compiler;
use crate::Target;
use std::collections::HashSet;
use std::fs;
use std::io;

use wasm_encoder::EntityType;
use wasm_encoder::ExportSection;
use wasm_encoder::{
    CodeSection, Function, FunctionSection, ImportSection, Instruction, MemorySection, MemoryType,
    Module, TypeSection, ValType,
};

pub(crate) struct Language {
    /// This contains the characterset from brainfk language
    pub char_set: HashSet<char>,
}

impl Language {
    pub fn new() -> Self {
        Self {
            char_set: HashSet::from(['<', '>', '+', '-', '.', ',', '[', ']']),
        }
    }
    pub fn code_gen(&self, code: &[char], target: &Target) -> Result<Vec<u8>, io::Error> {
        // Create a new module
        let mut module = Module::new();
        // Type section for void function
        let mut types = TypeSection::new();
        // Types for main function
        types.function([], []);
        match target {
            Target::Browser => {
                // Types for JS functions
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
        // ...
        let locals = vec![(3, ValType::I32)];
        let mut f = Function::new(locals);

        // <-- Linear Memory Model -->
        // ---------------------------
        // | IOV  and Tmp |  TAPE    |
        // 0-----------1024------4096|
        // TODO: Check if memory pointer invalid
        // TODO: Tape expansion
        f.instruction(&Instruction::I32Const(1024));
        f.instruction(&Instruction::LocalSet(0));

        // Code matching
        let mut fcount: u32 = 2;
        for symb in code {
            match symb {
                '<' => compiler::less_than(&mut f),
                '>' => compiler::greater_than(&mut f),
                '+' => compiler::plus(&mut f),
                '-' => compiler::minus(&mut f),
                ',' => compiler::comma(&mut f, &target),
                '.' => compiler::dot(&mut f, &target),
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
        // Cleanup
        f.instruction(&Instruction::End);

        codes.function(&f);
        module.section(&codes);

        let wasm_bytes = module.finish();
        Ok(wasm_bytes)
    }
    pub fn validate(&self, wasm_bytes: &[u8], output_file: &str) -> Result<(), io::Error> {
        wasmparser::validate(wasm_bytes).unwrap();
        fs::write(output_file, wasm_bytes)
    }
}
