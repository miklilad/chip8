pub struct Stack {
    data: [u16; 256],
    index: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: [0; 256],
            index: 0,
        }
    }

    pub fn push(&mut self, val: u16) {
        self.data[self.index] = val;
        self.index += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.index -= 1;
        let value = self.data[self.index];
        self.data[self.index] = 0;
        value
    }
}
