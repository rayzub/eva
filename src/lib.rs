use context::{ExecutionContext, Instruction}; use opcodes::Opcode;
pub mod opcodes;
pub mod context;
pub mod stack;
pub mod error;
pub mod executor;

pub fn new_dissasembler(bytecode_ref: &[u8]) -> ExecutionContext {
    let mut ixs = Vec::new();
    let mut i = 0;
    while i < bytecode_ref.len()  {
        if let Some(ix) = bytecode_ref.get(i).map(|op| Opcode::from(*op)) {
            let (ix_operand, new_indx) = ix.peek_operand(i, bytecode_ref);
            ixs.push(Instruction{ op: ix, operand: ix_operand, min_gas: ix.min_gas() });
            i = new_indx;
        } else {
            eprintln!("invalid bytecode malformed at index {}", i)
        }
    }
    for (indx, ix) in ixs.iter().enumerate() {
        println!("{} {}", format!("{:04}", indx), ix)
    }

    ExecutionContext::new(ixs)
}