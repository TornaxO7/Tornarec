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

use super::ArmOperand;

use std::convert::TryFrom;

pub fn get_normal_multiply(value: Word) -> ArmOperand {
    ArmOperand::Multiply {
        rs: Register::new(value, 8, 0b1111),
        rm: Register::new(value, 0, 0b1111),
        mul_type: MultiplyType::get_normal(value),
    }
}

pub fn get_long(value: Word) -> ArmOperand {
    ArmOperand::Multiply {
        rs: Register::new(value, 8, 0b1111),
        rm: Register::new(value, 0, 0b1111),
        mul_type: MultiplyType::get_long(value),
    }
}

pub fn get_halfword(value: Word) -> ArmOperand {
    ArmOperand::Multiply {
        rs: Register::new(value, 8, 0b1111),
        rm: Register::new(value, 0, 0b1111),
        mul_type: MultiplyType::get_halfword(value),
    }
}

pub fn get_word_halfword(value: Word) -> ArmOperand {
    ArmOperand::Multiply {
        rs: Register::new(value, 8, 0b1111),
        rm: Register::new(value, 0, 0b1111),
        mul_type: MultiplyType::get_word_halfword(value),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MultiplyType {
    Normal {
        rd: Register,
        rn: NormalMultiplyType,
    },
    Long {
        s: BitState,
        rdhi: u8,
        rdlo: u8,
    },
    Halfword {
        y: BitState,
        x: BitState,
        mul_type: HalfwordMultiplyType,
    },
    WordHalfword {
        rd: Register,
        y: BitState,
        mul_type: WordHalfwordMultiplyType,
    },
}

impl MultiplyType {
    pub fn get_normal(value: Word) -> Self {
        let rn = {
            let bit21 = BitState::new(value, 21);
            match bit21 {
                BitState::SET => NormalMultiplyType::MLA(Register::new(value, 12, 0b1111)),
                BitState::UNSET => {
                    sbz(value, 12, 0b1111);
                    NormalMultiplyType::MUL
                }
            }
        };

        Self::Normal {
            rd: Register::new(value, 16, 0b1111),
            rn,
        }
    }

    pub fn get_long(value: Word) -> Self {
        Self::Long {
            s: BitState::new(value, 20),
            rdhi: u8::try_from((value >> 16) & 0b1111).unwrap(),
            rdlo: u8::try_from((value >> 12) & 0b1111).unwrap(),
        }
    }

    pub fn get_halfword(value: Word) -> Self {
        Self::Halfword {
            y: BitState::new(value, 6),
            x: BitState::new(value, 5),
            mul_type: HalfwordMultiplyType::from(value),
        }
    }

    pub fn get_word_halfword(value: Word) -> Self {
        Self::WordHalfword {
            rd: Register::new(value, 16, 0b1111),
            y: BitState::new(value, 6),
            mul_type: WordHalfwordMultiplyType::from(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NormalMultiplyType {
    MLA(Register),
    MUL,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WordHalfwordMultiplyType {
    SMULWY,
    SMLAWY { rn: Register },
}

impl WordHalfwordMultiplyType {
    pub fn get_smulwy(value: Word) -> Self {
        sbz(value, 12, 0b1111);
        Self::SMULWY
    }

    pub fn get_smlawy(value: Word) -> Self {
        let rn = Register::new(value, 12, 0b1111);
        Self::SMLAWY { rn }
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

#[cfg(test)]
mod tests {
}
