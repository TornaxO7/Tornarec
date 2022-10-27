use crate::Addr;

pub const WRAM1_SIZE: usize = 32_000;
pub const WRAM2_SIZE: usize = 64_000;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq, Hash)]
pub enum WRamError {
    #[error("'(0)' is an invalid size for the WRam.")]
    InvalidCapacity(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WRam {
    pub storage: Vec<u8>,
}

impl WRam {
    pub fn new(storage: Vec<u8>) -> Result<Self, WRamError> {
        let cap = storage.capacity();

        if cap != WRAM1_SIZE && cap != WRAM2_SIZE {
            Err(WRamError::InvalidCapacity(cap))
        } else {
            Ok(Self { storage })
        }
    }

    pub fn fetch(addr: Addr) -> u32 {
        todo!()
    }
}
