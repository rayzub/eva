use std::fmt::{Display, Formatter};
use super::{stack::Stack, Opcode};

#[derive(Debug, Clone)]
pub struct Instruction {
    pub op: Opcode,
    pub operand: Option<Vec<u8>>,
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
    is_cold: bool,
    contract: String,
    slot: String,
    value: String,
}

pub struct Analyzer {
    pc: usize,
    stack: Stack,
    storage: Vec<Slot>,
    ixs: Vec<Instruction>,
    est_total_gas: usize,
}

// Create an instance of Disassembler
// Iterate through each returning an Option<Instruction> which has data of the specific instruction decoded
// Implement Display for Instruction so that we can pretty print


impl Analyzer {
    // Consume as this method primarily loads the bytecode into the disasembler
    pub fn new(ixs: Vec<Instruction>) -> Self {
        Self { 
            stack: Stack::new(), 
            storage: Vec::new(), 
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
            let encoded_operand = hex::encode(current_ix.operand.clone().unwrap());
            self.stack.push(encoded_operand)?
        } 
        if current_ix.op.is_pop() {
            self.stack.pop()?
        }
        
        self.est_total_gas += self.calc_gas(current_ix);
        self.pc += 1;
        Ok(())
    }
}

