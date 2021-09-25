use crate::ram::Address;
use crate::ram::DataBlock;

use core::ops::{Index, Range};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum RamError {
    #[error("[ROM ERROR]: Address `{0:X}` can't be accessed, because it's beyond the max size: {1:X}")]
    RamOverflow(Address, Address),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Ram {
    ram: Vec<u8>,
    max_size: Option<usize>,
}

impl Ram {
    pub fn new() -> Self {
        Self {
            ram: Vec::new(),
            max_size: None,
        }
    }

    pub fn load_data(&mut self, data: DataBlock, starting_address: Address) -> Result<(), RamError>{
        let last_address = starting_address.get_ref() + data.len();
        if self.ram.len() < last_address {
            self.ram.resize(last_address, 0);
        }

        if let Some(max_size) = self.max_size {
            if self.ram.len() > max_size {
                return Err(RamError::RamOverflow(Address::from(last_address), Address::from(max_size)));
            }
        }

        self.ram[starting_address.get()..].copy_from_slice(&data);
        Ok(())
    }

    pub fn set_max_size(&mut self, new_max_size: usize) {
        self.max_size = Some(new_max_size);
    }
}

impl Index<Range<usize>> for Ram
{
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.ram[index]
    }
}
