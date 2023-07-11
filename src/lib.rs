use analyzer::{Instruction, Analyzer};
use opcodes::{peek_operand, Opcode};

pub mod opcodes;
pub mod analyzer;
pub mod stack;


pub fn new_dissasembler<'a>(bytecode_str: &'a str) -> () {
    let mut ixs: Vec<Instruction> = Vec::new();
    let mut i = 0;
    while i < bytecode_str.len() {
        let b_op = &bytecode_str[i..i+2];  
        let ix = Opcode::from(b_op);
        let (ix_operand, new_indx) = peek_operand(i+2, ix, bytecode_str);
        ixs.push(Instruction{ op: ix, operand: ix_operand, total_gas: 0 });
        i = new_indx
    }
    for ix in ixs {
        println!("{}", ix)
    }
}