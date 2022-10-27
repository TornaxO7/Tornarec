const AMOUNT_BYTES: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FIFOUnit {
    pub queue1: [u8; AMOUNT_BYTES],
    pub queue2: [u8; AMOUNT_BYTES],
}

impl FIFOUnit {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for FIFOUnit {
    fn default() -> Self {
        Self {
            queue1: [0; AMOUNT_BYTES],
            queue2: [0; AMOUNT_BYTES],
        }
    }
}
