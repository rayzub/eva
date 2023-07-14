
use crate::analyzer::Instruction;

#[derive(Debug)]
pub enum StackError {
    StackTooDeep,
    StackUnderflow,
    DataExceedsLimit,
}
impl std::error::Error for StackError{}
impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackError::DataExceedsLimit => write!(f, "Stack data exceeds 256 bit limit!"),
            StackError::StackTooDeep => write!(f, "Stack cannot exceed limit of 1024!"),
            StackError::StackUnderflow => write!(f, "Stack cannot be less than 0!"),
        }
    }
}

#[derive(Debug)]
pub struct ExecuteError(Instruction);
impl std::error::Error for ExecuteError{}
impl std::fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str: &str = self.0.op.into();
        write!(f, "Error executing {}", op_str)
    }
}
