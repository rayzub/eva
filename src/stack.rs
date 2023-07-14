use crate::error;

const MAX_STACK_LIMIT: usize = 1024;
const MAX_DATA_LEN: usize = 32; // Vec<u8> size 32 bytes === 256 bits

pub struct Stack {
    data: Vec<String>
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

    pub fn push(&mut self, data: String) -> Result<(), error::StackError> {
        if self.data.len() + 1 > MAX_STACK_LIMIT {
            return Err(error::StackError::StackTooDeep)
        }

        if data.len() > MAX_DATA_LEN {
            return Err(error::StackError::DataExceedsLimit)
        }

        self.data.push(data);
        Ok(())
    }

    pub fn peek<'a>(&'a self, pos: usize) -> &String {
        self.data.get(pos).unwrap()
    } 
}