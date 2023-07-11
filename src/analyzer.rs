use std::fmt::{Display, Formatter, Result, format};

use super::{stack::Stack, opcodes::Opcode};


pub struct Instruction<'a> {
    pub op: Opcode,
    pub operand: Option<&'a str>,
    pub total_gas: i32,
}

impl<'a> Display for Instruction<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let op_str: &str = self.op.into();
        let operand_str = match self.operand {
            Some(ope_str) => format!("0x{}", ope_str),
            None => String::from(""),
        };
        write!(f, "{} {} - gas: {}", op_str, operand_str, self.total_gas)
    }
}



pub struct Analyzer<'a> {
    stack: Stack,
    bytecode: &'a [u8],
}

// Create an instance of Disassembler
// Iterate through each returning an Option<Instruction> which has data of the specific instruction decoded
// Implement Display for Instruction so that we can pretty print


impl<'a> Analyzer<'a> {
    // Consume as this method primarily loads the bytecode into the disasembler
    pub fn new() -> Analyzer<'a>{
        let stack = Stack::new();
        Analyzer { stack , bytecode: &[0], }
    }
}

