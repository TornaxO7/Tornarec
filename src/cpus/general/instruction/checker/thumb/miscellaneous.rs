use crate::cpus::general::{
    Instruction,
    BitState,
};

use super::ThumbCheckerError;

use std::convert::From;

// Tests are done in the thumb-checker-mod file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiscellaneousInstruction {
    AdjustStackPointer,
    PushPopRegisterList,
    SoftwareBreakpoint,
}

impl From<&Instruction> for MiscellaneousInstruction {
    fn from(instruction: &Instruction) -> Self {
        let bit9_10 = (instruction.val >> 9) & 0b11;
        let bit8 = BitState::from(instruction.val >> 8);
        let bit11 = BitState::from(instruction.val >> 11);

        match bit9_10 {
            0b00 if bit8.is_unset() && bit11.is_unset() => Self::AdjustStackPointer,
            0b10 => Self::PushPopRegisterList,
            0b11 if bit8.is_unset() && bit11.is_set() => Self::SoftwareBreakpoint,
            _ => unreachable!("{}", ThumbCheckerError::SuccessorInstruction(instruction.val)),
        }
    }
}
