pub const STACK_CAPACITY: usize = 1024;

pub struct STACK {
    pub stack_size: usize,
    pub data: [Option<i32>; STACK_CAPACITY],
}

impl STACK {
    pub fn push(&mut self, val: i32) {
        assert!(self.stack_size < STACK_CAPACITY, "[ERROR]: out of stack capacity");
        self.data[self.stack_size] = Some(val);
        self.stack_size += 1;
    }

    pub fn pop(&mut self) -> i32 {
        assert!(self.stack_size > 0, "[ERROR]: there is no to pop from stack");
        let popped_val: i32 = self.data[self.stack_size - 1].unwrap();
        self.data[self.stack_size - 1] = None;
        self.stack_size -= 1;
        return popped_val;
    }
}

impl Default for STACK {
    fn default() -> STACK {
        return STACK {
            stack_size: 0,
            data: [None; STACK_CAPACITY],
        };
    }
}
