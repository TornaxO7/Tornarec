use std::convert::From;

use crate::cpus::general::{BitState, Instruction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignedMultipliesOpcode {
    SMLA,
    SMLAW,
    SMULW,
    SMLAL,
    SMUL,
}

impl From<&Instruction> for SignedMultipliesOpcode {
    fn from(instruction: &Instruction) -> Self {
        let x_state = BitState::from(instruction.val >> 5);

        match  & 0b11 {
            0b00 => Self::SMLA,
            0b01 if x_state.is_unset() => Self::SMLAW,
            0b01 if x_state.is_set() => Self::SMULW,
            0b10 => Self::SMLAL,
            0b11 => Self::SMUL,
            other => unreachable!("[SignedMultipliesOpcode Opcode]: {:b} is an unknown opcode.", other),
        }
    }
}
