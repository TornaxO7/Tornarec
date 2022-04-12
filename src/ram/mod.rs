pub mod data_block;
pub mod data_types;

pub use data_block::DataBlock;

use std::{
    convert::TryFrom,
    ops::{
        Index,
        Range,
    },
};

pub type Address = u32;
pub type Byte = u8;
pub type Halfword = u16;
pub type Word = u32;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum RamError {
    #[error(
        "[RAM ERROR]: Address `{0:X}` can't be accessed, because it's beyond the max size: {1:X}"
    )]
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

    pub fn load_data(
        &mut self,
        data: DataBlock,
        starting_address: Address,
    ) -> Result<(), RamError> {
        let last_address = starting_address + data.size();

        if self.size() < last_address {
            self.ram.resize(usize::try_from(last_address).unwrap(), 0);
        }

        if let Some(max_size) = self.max_address {
            if self.size() > max_size {
                return Err(RamError::RamIndexOverflow(
                    Address::from(last_address),
                    Address::from(max_size),
                ));
            }
        }

        self.ram[usize::try_from(starting_address).unwrap()..].copy_from_slice(data.get_ref());
        Ok(())
    }

    pub fn set_max_address(&mut self, new_max_size: Address) {
        self.max_address = Some(new_max_size);
    }

    pub fn size(&self) -> u32 {
        match u32::try_from(self.ram.len()) {
            Ok(size) => size,
            Err(_) => panic!("{}", RamError::RamTooBig(self.ram.len())),
        }
    }
}

impl Index<Range<Address>> for Ram {
    type Output = [u8];

    fn index(&self, index: Range<Address>) -> &Self::Output {
        let range = Range {
            start: usize::try_from(index.start).unwrap(),
            end: usize::try_from(index.end).unwrap(),
        };

        &self.ram[range]
    }
}

#[cfg(test)]
mod tests {

    use super::{
        Ram,
        RamError,
    };
    use crate::ram::{
        Address,
        DataBlock,
    };

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
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x2, 0x3, 0x4,
            ],
            ..Ram::default()
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
            Err(RamError::RamIndexOverflow(
                Address::from(7),
                Address::from(5)
            ))
        );
    }

    #[test]
    fn get_range_of_ram() {
        let ram = Ram {
            ram: vec![0x1, 0x2, 0x3, 0x4],
            ..Ram::default()
        };

        let expected_output: [u8; 2] = [0x2, 0x3];

        let start = Address::from(1);
        let end = Address::from(3);

        assert_eq!(ram[start..end], expected_output);
    }
}
