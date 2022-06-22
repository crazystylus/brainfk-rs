use crate::Target;
use wasm_encoder::{BlockType, Function, Instruction, MemArg};

pub(crate) fn less_than(f: &mut Function) {
    // Move tape header left by 4 bytes
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::I32Const(4));
    f.instruction(&Instruction::I32Sub);
    f.instruction(&Instruction::LocalSet(0));
}

pub(crate) fn greater_than(f: &mut Function) {
    // Move tape header right by 4 bytes
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::I32Const(4));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::LocalSet(0));
}

pub(crate) fn plus(f: &mut Function) {
    // Increment value at tape header by 1
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::I32Load(MemArg {
        align: 0,
        memory_index: 0,
        offset: 0,
    }));
    f.instruction(&Instruction::I32Const(1));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::I32Store(MemArg {
        align: 0,
        memory_index: 0,
        offset: 0,
    }));
}

pub(crate) fn minus(f: &mut Function) {
    // Decrement value at tape header by 1
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::I32Load(MemArg {
        align: 0,
        memory_index: 0,
        offset: 0,
    }));
    f.instruction(&Instruction::I32Const(1));
    f.instruction(&Instruction::I32Sub);
    f.instruction(&Instruction::I32Store(MemArg {
        align: 0,
        memory_index: 0,
        offset: 0,
    }));
}

/// Output the byte at the data pointer
pub(crate) fn dot(f: &mut Function, target: &crate::Target) {
    match target {
        Target::Browser => {
            todo!();
        }
        Target::Wasi => {
            // Write IO Vector
            f.instruction(&Instruction::I32Const(0));
            f.instruction(&Instruction::LocalGet(0));
            f.instruction(&Instruction::I32Store(MemArg {
                align: 0,
                memory_index: 0,
                offset: 0,
            }));
            f.instruction(&Instruction::I32Const(4));
            f.instruction(&Instruction::I32Const(4));
            f.instruction(&Instruction::I32Store(MemArg {
                align: 0,
                memory_index: 0,
                offset: 0,
            }));
            f.instruction(&Instruction::I32Const(1)); // FD: Stdout
            f.instruction(&Instruction::I32Const(0)); // *iovs: 0
            f.instruction(&Instruction::I32Const(1)); // iovs_len
            f.instruction(&Instruction::I32Const(16)); // nbytes written
            f.instruction(&Instruction::Call(1)); // Call fd_write
            f.instruction(&Instruction::Drop);
        }
    };
}

/// Accept one byte of input, storing its value in the byte at the data pointer
pub(crate) fn comma(f: &mut Function, target: &crate::Target) {
    match target {
        Target::Browser => {
            todo!();
        }
        Target::Wasi => {
            // Write IO Vector
            f.instruction(&Instruction::I32Const(0));
            f.instruction(&Instruction::LocalGet(0));
            f.instruction(&Instruction::I32Store(MemArg {
                align: 0,
                memory_index: 0,
                offset: 0,
            }));
            f.instruction(&Instruction::I32Const(4));
            f.instruction(&Instruction::I32Const(1));
            f.instruction(&Instruction::I32Store(MemArg {
                align: 0,
                memory_index: 0,
                offset: 0,
            }));
            f.instruction(&Instruction::I32Const(0)); // FD: Stdin
            f.instruction(&Instruction::I32Const(0)); // *iovs: 0
            f.instruction(&Instruction::I32Const(1)); // iovs_len
            f.instruction(&Instruction::I32Const(16)); // nbytes written
            f.instruction(&Instruction::Call(0)); // Call fd_read
            f.instruction(&Instruction::Drop);
        }
    }
}

/// Loop Start
pub(crate) fn sq_start(f: &mut Function, _sig: u32) {
    // Skip the loop if condition not satisfied
    f.instruction(&Instruction::Block(BlockType::Empty));
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::I32Load(MemArg {
        align: 0,
        memory_index: 0,
        offset: 0,
    }));
    f.instruction(&Instruction::I32Eqz);
    f.instruction(&Instruction::BrIf(0));
    // Start Do..While loop
    f.instruction(&Instruction::Loop(BlockType::Empty));
}

/// Loop Stop
pub(crate) fn sq_end(f: &mut Function) {
    // Branch to loop on neqz
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::I32Load(MemArg {
        align: 0,
        memory_index: 0,
        offset: 0,
    }));
    f.instruction(&Instruction::I32Const(0));
    f.instruction(&Instruction::I32Ne);
    f.instruction(&Instruction::BrIf(0));
    // End Do..While Loop
    f.instruction(&Instruction::End);
    // End Block
    f.instruction(&Instruction::End);
}
