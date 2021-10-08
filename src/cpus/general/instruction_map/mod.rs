pub mod data_processing_immediate_shift;
pub mod miscellaneous1;
pub mod data_processing_register_shift;
pub mod miscellaneous2;
pub mod multiplies;
pub mod extra_load_stores;
pub mod data_processing_immediate;
pub mod move_immediate_to_status_register;
pub mod load_and_store_immediate_offset;
pub mod load_and_store_register_offset;
pub mod load_and_store_multiple;

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
