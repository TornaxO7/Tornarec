use crate::{
    cpus::general::instruction::arm::{
        BitState,
        Register,
    },
    ram::Word,
};

use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BLXType {
    Immediate { h: BitState, immed: u32 },
    Register(Register),
}

impl BLXType {
    pub fn get_immediate(value: Word) -> Self {
        let h = BitState::from(((value >> 24) & 0b1) != 0);
        let immed = value & 0b1111_1111_1111_1111_1111;

        Self::Immediate { h, immed }
    }

    pub fn get_register(value: Word) -> Self {
        let rm = Register::try_from(value & 0b1111).unwrap();

        Self::Register(rm)
    }
}

#[cfg(test)]
mod tests {

    use crate::cpus::general::instruction::arm::{
        BitState,
        Register,
    };

    use super::BLXType;

    #[test]
    fn test_get_immediate() {
        let value = 0b1111_1011_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            BLXType::Immediate {
                h: BitState::from(true),
                immed: 0b1111_1111_1111_1111_1111_1111,
            },
            BLXType::get_immediate(value)
        );
    }

    #[test]
    fn test_get_register() {
        let value = 0b0000_0001_0010_1111_1111_1111_0011_1111;

        assert_eq!(
            BLXType::Register(Register::from(0b1111)),
            BLXType::get_register(value)
        );
    }
}
