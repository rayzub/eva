use std::fmt::{Display, Formatter};
use primitive_types::H256;

use super::{stack::Stack, Opcode};

#[derive(Debug, Clone)]
pub struct Instruction {
    pub op: Opcode,
    pub operand: Option<[u8; 32]>,
    pub min_gas: usize,
}


impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let op_str: &str = self.op.into();
        let operand_str = match &self.operand {
            Some(b_operand) => format!("0x{}", hex::encode(b_operand)),
            None => String::from(""),
        };
        write!(f, "{} {} - gas: {}", op_str, operand_str, self.min_gas)
    }
}

struct Slot {
    contract: String,
    slot: H256,
    value: H256,
}


pub struct ExecutionContext {
    pc: usize,
    stack: Stack,
    memory: Vec<H256>,
    storage: Vec<Slot>,
    ixs: Vec<Instruction>,
    est_total_gas: usize,
}
impl ExecutionContext {
    // Consume as this method primarily loads the bytecode into the disasembler
    pub fn new(ixs: Vec<Instruction>) -> Self {
        Self { 
            stack: Stack::new(), 
            storage: Vec::new(),
            memory: Vec::new(), 
            ixs, 
            est_total_gas: 0, 
            pc: 0 
        }
    }

    pub fn calc_gas(&self, ix: &Instruction) -> usize {
        let min_gas = ix.op.min_gas();
        match ix.op {
           Opcode::SLOAD if self.storage.iter().find(|slot| slot.value == *self.stack.peek(1)).is_some() => 2100,
           Opcode::SLOAD => 100,
           Opcode::EXP => min_gas + (50 * self.stack.peek(2).as_bytes().len()),
            _ => min_gas,
        }
    }

    pub fn step(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        let current_ix = self.ixs.get(self.pc).unwrap();
        if current_ix.op.is_push() { 
            let encoded_operand = current_ix.operand.clone().unwrap();
            self.stack.push(encoded_operand.into())?
        } 
        if current_ix.op.is_pop() {
            self.stack.pop()?
        }
        
        self.est_total_gas += self.calc_gas(current_ix);
        self.pc += 1;
        Ok(())
    }
}

