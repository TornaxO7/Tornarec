use crate::{
    cpus::general::instruction::arm::{
        types::{
            sbz,
            Register,
        },
        BitState,
    },
    ram::Word,
};

use std::convert::TryFrom;

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
    ArmOperand::MiscLoadStore {
        p: BitState::new(value, 24),
        u: BitState::new(value, 23),
        w: BitState::new(value, 21),
        rn: Register::new(value, 16, 0b1111),
        rd: Register::new(value, 12, 0b1111),
        s: BitState::new(value, 6),
        h: BitState::new(value, 5),
        offset: AddressingMode3::from(value),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AddressingMode3 {
    Immediate { immedh: u8, immedl: u8 },
    Register(Register),
}

impl AddressingMode3 {
    pub fn get_immediate(value: Word) -> Self {
        Self::Immediate {
            immedh: u8::try_from((value >> 8) & 0b1111).unwrap(),
            immedl: u8::try_from(value & 0b1111).unwrap(),
        }
    }

    pub fn get_register(value: Word) -> Self {
        let register = Register::new(value, 0, 0b1111);

        sbz(value, 8, 0b1111);
        Self::Register(register)
    }
}

impl From<Word> for AddressingMode3 {
    fn from(value: Word) -> Self {
        let bit22 = BitState::new(value, 22);
        match bit22 {
            BitState::SET => Self::get_immediate(value),
            BitState::UNSET => Self::get_register(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpus::general::instruction::arm::{
        operand::{
            misc_load_store::AddressingMode3,
            ArmOperand,
        },
        types::Register,
        BitState,
    };

    use super::get_operand;

    #[test]
    fn test_addressing_mode3_get_immediate() {
        let value = 0b0000_0001_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            AddressingMode3::Immediate {
                immedh: 0b1111,
                immedl: 0b1111,
            },
            AddressingMode3::from(value)
        );
    }

    #[test]
    fn test_addressing_mode3_get_register() {
        let value = 0b0000_0001_1011_1111_1111_0000_1111_1111;

        assert_eq!(
            AddressingMode3::Register(Register::from(0b1111)),
            AddressingMode3::get_register(value)
        );
    }

    #[test]
    fn test_addressing_mode3_get_register_sbz() {
        let value = 0b0000_0001_1011_1111_1111_1111_1111_1111;
        AddressingMode3::get_register(value);
    }

    #[test]
    fn test_get_operand_immediate() {
        let value = 0b0000_0001_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmOperand::MiscLoadStore {
                p: BitState::SET,
                u: BitState::SET,
                w: BitState::SET,
                rn: Register::from(0b1111),
                rd: Register::from(0b1111),
                s: BitState::SET,
                h: BitState::SET,
                offset: AddressingMode3::from(value),
            },
            get_operand(value)
        );
    }
}
