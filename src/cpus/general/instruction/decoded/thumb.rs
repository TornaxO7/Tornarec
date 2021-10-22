use crate::cpus::general::instruction::{
    checker::{
        ThumbInstructionChecker, 
        thumb::miscellaneous::MiscellaneousInstruction
    },
    encodings::thumb::{
        ShiftByImmediate,
        AddSubtractRegister,
        AddSubtractImmediate,
        AddSubtractCompareMoveImmediate,
        DataProcessingRegister,
        SpecialDataProcessing,
        UnconditionalBranch,
        BranchExchangeInstructionSet,
        LoadFromLiteralPool,
        LoadStoreRegisterOffset,
        LoadStoreWordByteImmediateOffset,
        LoadStoreHalfwordImmediateOffset, 
        LoadStoreToFromStack,
        AddToSpOrPc,
        LoadStoreMultiple,
        ConditionalBranch,
        SoftwareInterrupt,
        BlxSuffix,
        BlOrBlxPrefix,
        BlSuffix,

        miscellaneous::{
            AdjustStackPointer,
            PushPopRegisterList,
            SoftwareBreakpoint,
        },
    },
    Instruction,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThumbDecoder {
    ShiftByImmediate(ShiftByImmediate),
    AddSubtractRegister(AddSubtractRegister),
    AddSubtractImmediate(AddSubtractImmediate),
    AddSubtractCompareMoveImmediate(AddSubtractCompareMoveImmediate),
    DataProcessingRegister(DataProcessingRegister),
    SpecialDataProcessing(SpecialDataProcessing),
    UnconditionalBranch(UnconditionalBranch),
    BranchExchangeInstructionSet(BranchExchangeInstructionSet),
    LoadFromLiteralPool(LoadFromLiteralPool),
    LoadStoreRegisterOffset(LoadStoreRegisterOffset),
    LoadStoreWordByteImmediateOffset(LoadStoreWordByteImmediateOffset),
    LoadStoreHalfwordImmediateOffset(LoadStoreHalfwordImmediateOffset),
    LoadStoretoFromStack(LoadStoreToFromStack),
    AddToSpOrPc(AddToSpOrPc),
    LoadStoreMultiple(LoadStoreMultiple),
    ConditionalBranch(ConditionalBranch),
    UndefinedInstruction,
    SoftwareInterrupt(SoftwareInterrupt),
    BlxSuffix(BlxSuffix),
    BlOrBlxPrefix(BlOrBlxPrefix),
    BlSuffix(BlSuffix),

    // miscellaneous instructions,
    AdjustStackPointer(AdjustStackPointer),
    PushPopRegisterList(PushPopRegisterList),
    SoftwareBreakpoint(SoftwareBreakpoint),
}

impl From<&Instruction> for ThumbDecoder {
    fn from(instruction: &Instruction) -> Self {
        match ThumbInstructionChecker::from(instruction) {
            ThumbInstructionChecker::ShiftByImmediate =>
                Self::ShiftByImmediate(ShiftByImmediate::from(instruction)),
            ThumbInstructionChecker::AddSubtractRegister =>
                Self::AddSubtractRegister(AddSubtractRegister::from(instruction)),
            ThumbInstructionChecker::AddSubtractImmediate =>
                Self::AddSubtractImmediate(AddSubtractImmediate::from(instruction)),
            ThumbInstructionChecker::AddSubtractCompareMoveImmediate =>
                Self::AddSubtractCompareMoveImmediate(AddSubtractCompareMoveImmediate::from(instruction)),
            ThumbInstructionChecker::DataProcessingRegister =>
                Self::DataProcessingRegister(DataProcessingRegister::from(instruction)),
            ThumbInstructionChecker::SpecialDataProcessing =>
                Self::SpecialDataProcessing(SpecialDataProcessing::from(instruction)),
            ThumbInstructionChecker::BranchExchangeInstructionSet =>
                Self::BranchExchangeInstructionSet(BranchExchangeInstructionSet::from(instruction)),
            ThumbInstructionChecker::LoadFromLiteralPool =>
                Self::LoadFromLiteralPool(LoadFromLiteralPool::from(instruction)),
            ThumbInstructionChecker::LoadStoreRegisterOffset =>
                Self::LoadStoreRegisterOffset(LoadStoreRegisterOffset::from(instruction)),
            ThumbInstructionChecker::LoadStoreWordByteImmediateOffset =>
                Self::LoadStoreWordByteImmediateOffset(LoadStoreWordByteImmediateOffset::from(instruction)),
            ThumbInstructionChecker::LoadStoreHalfwordImmediateOffset =>
                Self::LoadStoreHalfwordImmediateOffset(LoadStoreHalfwordImmediateOffset::from(instruction)),
            ThumbInstructionChecker::LoadStoreToFromStack =>
                Self::LoadStoretoFromStack(LoadStoreToFromStack::from(instruction)),
            ThumbInstructionChecker::AddToSpOrPc =>
                Self::AddToSpOrPc(AddToSpOrPc::from(instruction)),
            ThumbInstructionChecker::Miscellaneous(miscellaneous_instruction) => match miscellaneous_instruction {
                MiscellaneousInstruction::AdjustStackPointer =>
                    Self::AdjustStackPointer(AdjustStackPointer::from(instruction)),
                MiscellaneousInstruction::PushPopRegisterList =>
                    Self::PushPopRegisterList(PushPopRegisterList::from(instruction)),
                MiscellaneousInstruction::SoftwareBreakpoint =>
                    Self::SoftwareBreakpoint(SoftwareBreakpoint::from(instruction)),
            },
            ThumbInstructionChecker::LoadStoreMultiple =>
                Self::LoadStoreMultiple(LoadStoreMultiple::from(instruction)),
            ThumbInstructionChecker::ConditionalBranch =>
                Self::ConditionalBranch(ConditionalBranch::from(instruction)),
            ThumbInstructionChecker::UndefinedInstruction =>
                Self::UndefinedInstruction,
            ThumbInstructionChecker::SoftwareInterrupt =>
                Self::SoftwareInterrupt(SoftwareInterrupt::from(instruction)),
            ThumbInstructionChecker::UnconditionalBranch =>
                Self::UnconditionalBranch(UnconditionalBranch::from(instruction)),
            ThumbInstructionChecker::BlxSuffix =>
                Self::BlxSuffix(BlxSuffix::from(instruction)),
            ThumbInstructionChecker::BlOrBlxPrefix =>
                Self::BlOrBlxPrefix(BlOrBlxPrefix::from(instruction)),
            ThumbInstructionChecker::BlSuffix =>
                Self::BlSuffix(BlSuffix::from(instruction)),
        }
    }
}
