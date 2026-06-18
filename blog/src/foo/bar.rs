#[derive(Default)]
pub struct Baz {
    pub bah: Option<i32>,
}

impl Baz {
    pub fn new(x: i32) -> Self {
        Self { bah: Some(x) }
    }

    pub fn set(&mut self, y: i32) {
        self.bah = Some(y);
    }

    pub fn get_ref(&self) -> Option<&i32> {
        self.bah.as_ref()
    }
}
