pub mod branch;
pub mod data_processing;
pub mod instruction_map_trait;
pub mod encoding_types;
pub mod multiply;
pub mod cpsr_access;

pub use data_processing::DataProcessing;
pub use branch::Branch;
pub use multiply::Multiply;
pub use cpsr_access::CpsrAccess;
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
    Multiply(Multiply),
    CpsrAccess(CpsrAccess),
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
            Self::Branch(Branch::from(instruction))
        } else if DataProcessing::is_matching(instruction) {
            Self::DataProcessing(DataProcessing::from(instruction))
        } else if Multiply::is_matching(instruction) {
            Self::Multiply(Multiply::from(instruction))
        } else if CpsrAccess::is_matching(instruction) {
            Self::CpsrAccess(CpsrAccess::from(instruction))
        } else {
            Self::Noop
        }
    }
}
