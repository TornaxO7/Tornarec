use crate::cpus::general::instruction::{
    decode::DecodeData,
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
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThumbDecode {
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

impl<'a> From<DecodeData<'a>> for ThumbDecode {
    fn from(decode_data: DecodeData<'a>) -> Self {
        match ThumbInstructionChecker::from(&decode_data.instruction) {
            ThumbInstructionChecker::ShiftByImmediate =>
                Self::ShiftByImmediate(ShiftByImmediate::from(decode_data)),
            ThumbInstructionChecker::AddSubtractRegister =>
                Self::AddSubtractRegister(AddSubtractRegister::from(decode_data)),
            ThumbInstructionChecker::AddSubtractImmediate =>
                Self::AddSubtractImmediate(AddSubtractImmediate::from(decode_data)),
            ThumbInstructionChecker::AddSubtractCompareMoveImmediate =>
                Self::AddSubtractCompareMoveImmediate(AddSubtractCompareMoveImmediate::from(decode_data)),
            ThumbInstructionChecker::DataProcessingRegister =>
                Self::DataProcessingRegister(DataProcessingRegister::from(decode_data)),
            ThumbInstructionChecker::SpecialDataProcessing =>
                Self::SpecialDataProcessing(SpecialDataProcessing::from(decode_data)),
            ThumbInstructionChecker::BranchExchangeInstructionSet =>
                Self::BranchExchangeInstructionSet(BranchExchangeInstructionSet::from(decode_data)),
            ThumbInstructionChecker::LoadFromLiteralPool =>
                Self::LoadFromLiteralPool(LoadFromLiteralPool::from(decode_data)),
            ThumbInstructionChecker::LoadStoreRegisterOffset =>
                Self::LoadStoreRegisterOffset(LoadStoreRegisterOffset::from(decode_data)),
            ThumbInstructionChecker::LoadStoreWordByteImmediateOffset =>
                Self::LoadStoreWordByteImmediateOffset(LoadStoreWordByteImmediateOffset::from(decode_data)),
            ThumbInstructionChecker::LoadStoreHalfwordImmediateOffset =>
                Self::LoadStoreHalfwordImmediateOffset(LoadStoreHalfwordImmediateOffset::from(decode_data)),
            ThumbInstructionChecker::LoadStoreToFromStack =>
                Self::LoadStoretoFromStack(LoadStoreToFromStack::from(decode_data)),
            ThumbInstructionChecker::AddToSpOrPc =>
                Self::AddToSpOrPc(AddToSpOrPc::from(decode_data)),
            ThumbInstructionChecker::Miscellaneous(miscellaneous_instruction) => match miscellaneous_instruction {
                MiscellaneousInstruction::AdjustStackPointer =>
                    Self::AdjustStackPointer(AdjustStackPointer::from(decode_data)),
                MiscellaneousInstruction::PushPopRegisterList =>
                    Self::PushPopRegisterList(PushPopRegisterList::from(decode_data)),
                MiscellaneousInstruction::SoftwareBreakpoint =>
                    Self::SoftwareBreakpoint(SoftwareBreakpoint::from(decode_data)),
            },
            ThumbInstructionChecker::LoadStoreMultiple =>
                Self::LoadStoreMultiple(LoadStoreMultiple::from(decode_data)),
            ThumbInstructionChecker::ConditionalBranch =>
                Self::ConditionalBranch(ConditionalBranch::from(decode_data)),
            ThumbInstructionChecker::UndefinedInstruction =>
                Self::UndefinedInstruction,
            ThumbInstructionChecker::SoftwareInterrupt =>
                Self::SoftwareInterrupt(SoftwareInterrupt::from(decode_data)),
            ThumbInstructionChecker::UnconditionalBranch =>
                Self::UnconditionalBranch(UnconditionalBranch::from(decode_data)),
            ThumbInstructionChecker::BlxSuffix =>
                Self::BlxSuffix(BlxSuffix::from(decode_data)),
            ThumbInstructionChecker::BlOrBlxPrefix =>
                Self::BlOrBlxPrefix(BlOrBlxPrefix::from(decode_data)),
            ThumbInstructionChecker::BlSuffix =>
                Self::BlSuffix(BlSuffix::from(decode_data)),
        }
    }
}
