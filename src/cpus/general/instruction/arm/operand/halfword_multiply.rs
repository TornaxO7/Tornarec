use crate::{
    cpus::general::instruction::arm::{types::Register, BitState},
    ram::Word,
};

use std::convert::TryFrom;

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
    ArmOperand::HalfwordMultiply {
        rs: Register::new(value, 8, 0b1111),
        y: BitState::new(value, 6),
        x: BitState::new(value, 5),
        rm: Register::new(value, 0, 0b1111),
        mul_type: HalfwordMultiplyType::from(value),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HalfwordMultiplyType {
    SMULXY { rd: Register },
    SMLAXY { rd: Register, rn: Register },
    SMLALXY { rdhi: u8, rdlo: u8 },
}

impl HalfwordMultiplyType {
    pub fn get_smul(value: Word) -> Self {
        Self::SMULXY {
            rd: Register::new(value, 16, 0b1111),
        }
    }

    pub fn get_smla(value: Word) -> Self {
        Self::SMLAXY {
            rd: Register::new(value, 16, 0b1111),
            rn: Register::new(value, 12, 0b1111),
        }
    }

    pub fn get_smlal(value: Word) -> Self {
        Self::SMLALXY {
            rdhi: u8::try_from((value >> 16) & 0b1111).unwrap(),
            rdlo: u8::try_from((value >> 12) & 0b1111).unwrap(),
        }
    }
}

impl From<Word> for HalfwordMultiplyType {
    fn from(value: Word) -> Self {
        let bit27_20 = (value >> 20) & 0b1111_1111;

        match bit27_20 {
            0b0001_0110 => Self::get_smul(value),
            0b0001_0000 => Self::get_smla(value),
            0b0001_0100 => Self::get_smlal(value),
            _ => unreachable!(),
        }
    }
}
