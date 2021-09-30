use crate::cpus::general::instruction::Instruction;

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prefetch {
    Success(Instruction),
    Invalid,
}

impl From<Instruction> for Prefetch {
    fn from(data_type: Instruction) -> Self {
        Self::Success(data_type)
    }
}

impl Default for Prefetch {
    fn default() -> Self {
        Self::Success(Instruction::default())
    }
}
