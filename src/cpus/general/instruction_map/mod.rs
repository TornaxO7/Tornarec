pub mod instruction_group;
pub mod branch;
pub mod data_processing;
pub mod instruction_map_trait;
pub mod encoding_types;
pub mod multiply;
pub mod cpsr_access;
pub mod load_and_store;
pub mod load_and_store_multiple;

pub use data_processing::DataProcessing;
pub use branch::Branch;
pub use multiply::Multiply;
pub use cpsr_access::CpsrAccess;
pub use instruction_map_trait::InstructionMapTrait;
pub use instruction_group::InstructionGroup;


use crate::cpus::general::{
    exception::Exception,
    instruction::Instruction,
};

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionMap {
    Branch(Branch),
    DataProcessing(DataProcessing),
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
        match InstructionGroup::from(instruction) {
            InstructionGroup::Branch => (),
            InstructionGroup::DataProcessing => (),
            InstructionGroup::Multiply => (),
            InstructionGroup::StatusRegisterAccess => (),
            InstructionGroup::LoadAndStore => (),
            InstructionGroup::LoadAndStoreMultiple => (),
            InstructionGroup::Semaphore => (),
            InstructionGroup::ExceptionGenerating => (),
            InstructionGroup::Coprocessor => (),
        }
    }
}
