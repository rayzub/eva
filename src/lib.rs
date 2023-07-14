use core::panic;

use analyzer::{Analyzer, Instruction}; use opcodes::Opcode;
pub mod opcodes;
pub mod analyzer;
pub mod stack;
pub mod mem;
pub mod error;

pub fn new_dissasembler<'a>(bytecode_str: Vec<u8>) -> Analyzer {
    let mut ixs = Vec::new();
    let mut i = 0;
    while i < bytecode_str.len()  {
        if let Some(ix) = bytecode_str.get(i).map(|op| Opcode::from(*op)) {
            let (ix_operand, new_indx) = ix.peek_operand(i, &bytecode_str);
            ixs.push(Instruction{ op: ix, operand: ix_operand, min_gas: ix.min_gas() });
            i = new_indx;
        } else {
            panic!("invalid bytecode malformed at index {}", i)
        }

    }
    let analyzer = Analyzer::new(ixs);
    analyzer 
}