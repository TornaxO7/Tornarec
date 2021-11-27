#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dynarec {
    pub buffer: Vec<u8>,
}

impl Dynarec {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }

    /// Let's get started!
    pub fn exec(&self) {
        unsafe {
            self.buffer();
        }
    }
}
