use crate::cpus::general::Instruction;

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThumbInstructionChecker {
    ShiftByImmediate,
    AddSubtractRegister,
    AddSubtractImmediate,
    AddSubtractCompareMoveImmediate,
    DataProcessingRegister,
    SpecialDataProcessing,
    BranchExchangeInstructionset,
    LoadFromLiteralPool,
    LoadStoreRegisterOffset,
    LoadStoreWordByteImmediateOffset,
    LoadStoreHalfwordImmediateOffset,
    LoadStoreToFromSTack,
    AddToSpOrPc,
    LoadStoreMultiple,
    ConditionalBranch,
    UndefinedInstruction,
    SoftwareInterrupt,
    BLXSuffix,
    BlOrBlxPrefix,
    BlSuffix,
}

impl From<&Instruction> for ThumbInstructionChecker {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();
    }
}
