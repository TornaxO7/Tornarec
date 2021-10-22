pub mod data_block;
pub mod address;
pub mod data_types;

pub use address::Address;
pub use data_block::DataBlock;

use std::{
    ops::{Index, Range},
    convert::TryFrom,
};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum RamError {
    #[error("[RAM ERROR]: Address `{0:X}` can't be accessed, because it's beyond the max size: {1:X}")]
    RamIndexOverflow(Address, Address),

    #[error("[RAM ERROR]: Ram is too large: '{0}' bits long.")]
    RamTooBig(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Ram {
    ram: Vec<u8>,
    max_address: Option<u32>,
}

impl Ram {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_data(&mut self, data: DataBlock, starting_address: Address) -> Result<(), RamError>{
        let last_address = starting_address.get_as_u32() + data.size();

        if self.size() < last_address {
            self.ram.resize(usize::try_from(last_address).unwrap(), 0);
        }

        if let Some(max_size) = self.max_address {
            if self.size() > max_size {
                return Err(RamError::RamIndexOverflow(Address::from(last_address), Address::from(max_size)));
            }
        }

        self.ram[usize::try_from(starting_address.get_as_u32()).unwrap()..].copy_from_slice(data.get_ref());
        Ok(())
    }

    pub fn set_max_address(&mut self, new_max_size: Address) {
        self.max_address = Some(new_max_size.get_as_u32());
    }

    pub fn size(&self) -> u32 {
        match u32::try_from(self.ram.len()) {
            Ok(size) => size,
            Err(_) => panic!("{}", RamError::RamTooBig(self.ram.len())),
        }
    }
}

impl Index<Range<usize>> for Ram {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.ram[index]
    }
}

impl Index<Range<u32>> for Ram {
    type Output = [u8];

    fn index(&self, index: Range<u32>) -> &Self::Output {

        let range = Range {
            start: usize::try_from(index.start).unwrap(),
            end: usize::try_from(index.end).unwrap(),
        };

        &self.ram[range]
    }
}

impl Index<Range<i32>> for Ram {
    type Output = [u8];

    fn index(&self, index: Range<i32>) -> &Self::Output {

        let range = Range {
            start: usize::try_from(index.start).unwrap(),
            end: usize::try_from(index.end).unwrap(),
        };

        &self.ram[range]
    }
}

#[cfg(test)]
mod tests {

    use super::{Ram, RamError};
    use crate::ram::{Address, DataBlock};
    
    #[test]
    fn new() {
        let expected_ram = Ram {
            ram: Vec::new(),
            max_address: None,
        };
        let ram = Ram::new();

        assert_eq!(ram, expected_ram);
    }

    #[test]
    fn load_data() {
        let mut ram = Ram::new();
        let expected_ram = Ram {
            ram: vec![
                // 10 padding, because starting address is 0x10
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 
                0x1, 0x2, 0x3, 0x4],
            .. Ram::default()
        };
        
        let data = DataBlock::from(vec![0x1, 0x2, 0x3, 0x4]);
        let start = Address::from(10);

        ram.load_data(data, start).unwrap();

        assert_eq!(ram, expected_ram);
    }

    #[test]
    fn fail_load_data_due_to_max_size() {
        let mut ram = Ram::new();
        ram.set_max_address(Address::from(5));

        let result = ram.load_data(DataBlock::from(vec![1]), Address::from(6));

        assert_eq!(
            result,
            Err(RamError::RamIndexOverflow(Address::from(7), Address::from(5)))
        );
    }

    #[test]
    fn get_range_of_ram() {
        let ram = Ram {
            ram: vec![0x1, 0x2, 0x3, 0x4],
            .. Ram::default()
        };

        let expected_output: [u8; 2] = [0x2, 0x3];

        assert_eq!(ram[1..3], expected_output);
    }
}
