use crate::{cpus::general::instruction::arm::{types::{Register, sbz}, BitState}, ram::Word};

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
    ArmOperand::WordHalfwordMultiply {
        rd: Register::new(value, 16, 0b1111),
        rs: Register::new(value, 8, 0b1111),
        y: BitState::new(value, 6),
        rm: Register::new(value, 0, 0b1111),
        mul_type: WordHalfwordMultiplyType::from(value),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WordHalfwordMultiplyType {
    SMULWY,
    SMLAWY {
        rn: Register,
    }
}

impl WordHalfwordMultiplyType {
    pub fn get_smulwy(value: Word) -> Self {
        sbz(value, 12, 0b1111);
        Self::SMULWY
    }

    pub fn get_smlawy(value: Word) -> Self {
        let rn = Register::new(value, 12, 0b1111);
        Self::SMLAWY {
            rn
        }
    }
}

impl From<Word> for WordHalfwordMultiplyType {
    fn from(value: Word) -> Self {
        let bit5 = BitState::new(value, 5);
        match bit5 {
            BitState::SET => Self::get_smulwy(value),
            BitState::UNSET => Self::get_smlawy(value),
        }
    }
}
