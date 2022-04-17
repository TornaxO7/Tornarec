use crate::{
    cpus::general::instruction::arm::{
        types::Register,
        BitState,
    },
    ram::Word,
};

use std::convert::TryFrom;

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
    ArmOperand::LoadStoreWordOrByte {
        p: BitState::new(value, 24),
        u: BitState::new(value, 23),
        b: BitState::new(value, 22),
        w: BitState::new(value, 21),
        rn: Register::new(value, 16, 0b1111),
        rd: Register::new(value, 12, 0b1111),
        offset: AddressingMode2::from(value),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode2 {
    Immediate(u16),
    Register {
        shift_imm: u8,
        shift: u8,
        rm: Register,
    },
}

impl AddressingMode2 {
    pub fn get_immediate(value: Word) -> Self {
        let immed12 = u16::try_from(value & 0b1111_1111_1111).unwrap();
        Self::Immediate(immed12)
    }

    pub fn get_register(value: Word) -> Self {
        Self::Register {
            shift_imm: u8::try_from((value >> 7) & 0b1_1111).unwrap(),
            shift: u8::try_from((value >> 5) & 0b11).unwrap(),
            rm: Register::new(value, 0, 0b1111),
        }
    }
}

impl From<Word> for AddressingMode2 {
    fn from(value: Word) -> Self {
        let bit25 = BitState::new(value, 25);
        match bit25 {
            BitState::SET => Self::get_register(value),
            BitState::UNSET => Self::get_immediate(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpus::general::instruction::arm::{
        operand::load_store_word_byte::AddressingMode2,
        types::Register,
    };

    #[test]
    fn test_addressing_mode2_immediate() {
        let value = 0b0000_0101_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            AddressingMode2::Immediate(0b1111_1111_1111),
            AddressingMode2::get_immediate(value)
        );
    }

    #[test]
    fn test_addressing_mode2_register() {
        let value = 0b0000_0111_1111_1111_1111_1111_1110_1111;

        assert_eq!(
            AddressingMode2::Register {
                shift_imm: 0b1_1111,
                shift: 0b11,
                rm: Register::from(0b1111),
            },
            AddressingMode2::get_register(value)
        );
    }
}
