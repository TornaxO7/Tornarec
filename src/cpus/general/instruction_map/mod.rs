pub mod branch;
pub mod data_processing;
pub mod instruction_map_trait;
pub mod encoding_types;

pub use data_processing::DataProcessing;
pub use branch::Branch;
pub use instruction_map_trait::InstructionMapTrait;

use crate::cpus::general::{
    exception::Exception,
    instruction::Instruction,
};

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionMap {
    DataProcessing(DataProcessing),
    Branch(Branch),
    Exception(Exception),
    Noop,
}

impl Default for InstructionMap {
    fn default() -> Self {
        Self::Noop
    }
}

impl From<&Instruction> for InstructionMap {
    fn from(instruction: &Instruction) -> Self {
        if Branch::is_matching(instruction)  {
            Self::Branch(Branch::match_instruction(instruction))
        } else if DataProcessing::is_matching(instruction) {
            Self::DataProcessing(DataProcessing::match_instruction(instruction))
        } else {
            Self::Noop
        }
    }
}
