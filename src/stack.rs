

const MAX_WORD_COUNT: i32 = 1024;
const MAX_WORD_SIZE:  i32 = 256;

pub struct Stack {
    data: Vec<u8>
}

impl Stack {
    pub fn new() -> Stack {
        Stack { data: Vec::new() }
    }
}
