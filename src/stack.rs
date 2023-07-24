use crate::error;
use primitive_types::H256;


const MAX_STACK_LIMIT: usize = 1024;
pub struct Stack {
    data: Vec<H256>
}

impl Stack { 
    pub fn new() -> Self {
        Self {
            data: Vec::new()
        }
    } 

    pub fn pop(&mut self) -> Result<(), error::StackError> {
        match self.data.pop() {
            None => Err(error::StackError::StackUnderflow),
            _ => Ok(())
        }
    }

    pub fn push(&mut self, data: H256) -> Result<(), error::StackError> {
        if self.data.len() + 1 > MAX_STACK_LIMIT {
            return Err(error::StackError::StackTooDeep)
        }

        // .try_into() in peek_operand will already enforce 32 element size lim
        //if data.len() > MAX_DATA_LEN {
        //    return Err(error::StackError::DataExceedsLimit)
        //}

        self.data.push(data);
        Ok(())
    }

    pub fn peek(&self, pos: usize) -> &H256 {
        self.data.get(pos).unwrap()
    } 
}