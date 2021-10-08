pub mod data_processing_immediate_shift;
pub mod miscellaneous1;

use std::convert::From;

use crate::cpus::general::instruction::Instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionMap {
    Noop
}

impl From<&Instruction> for InstructionMap {
    fn from(_instruction: &Instruction) -> Self {
        Self::Noop
    }
}

impl Default for InstructionMap {
    fn default() -> Self {
        Self::Noop
    }
}
