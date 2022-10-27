use crate::Addr;

pub const PSRAM_SIZE: usize = 4_000_000;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PSRam {
    storage: [u8; PSRAM_SIZE],
}

impl PSRam {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fetch(addr: Addr) -> u16 {
        todo!()
    }
}

impl Default for PSRam {
    fn default() -> Self {
        Self {
            storage: [0; PSRAM_SIZE],
        }
    }
}
