pub struct Counter {
    value: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn starting_at(s: u32) -> Self {
        Self { value: s }
    }

    pub fn increment(&mut self, inc: u32) {
        self.value += inc;
    }

    pub fn val(&mut self) -> u32 {
        let output = self.value;
        self.value += 1;
        output
    }

    pub fn get(&self) -> u32 {
        self.value
    }
}
