use crate::cpus::general::instruction::Instruction;

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionGroup {
    Branch,
    DataProcessing,
    Multiply,
    StatusRegisterAccess,
    LoadAndStore,
    LoadAndStoreMultiple,
    Semaphore,
    ExceptionGenerating,
    Coprocessor,
}

impl From<&Instruction> for InstructionGroup {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

    }
}
