use crate::cpus::general::instruction::{
    encodings::arm::{
        DataProcessingImmediateShift,
        Miscellaneous1,
        DataProcessingRegisterShift,
        Miscellaneous2,
        Multiplies,
        ExtraLoadAndStores,
        DataProcessingImmediate,
        MoveImmediateToStatusRegister,
        LoadAndStoreImmediateOffset,
        LoadAndStoreRegisterOffset,
        LoadAndStoreMultiple,
        BranchAndBranchWithLink,
        CoprocessorLoadAndStoreAndDoubleRegisterTransfers,
        CoprocessorDataProcessing,
        CoprocessorRegisterTransfers,
    },
    checker::ArmInstructionChecker,
    Instruction,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmInstruction {
    DataProcessingImmediateShift(DataProcessingImmediateShift),
    Miscellaneous1(Miscellaneous1),
    DataProcessingRegisterShift(DataProcessingRegisterShift),
    Miscellaneous2(Miscellaneous2),
    Multiplies(Multiplies),
    ExtraLoadAndStores(ExtraLoadAndStores),
    DataProcessingImmediate(DataProcessingImmediate),
    UndefinedInstruction,
    MoveImmediateToStatusRegister(MoveImmediateToStatusRegister),
    LoadAndStoreImmediateOffset(LoadAndStoreImmediateOffset),
    LoadAndStoreRegisterOffset(LoadAndStoreRegisterOffset),
    MediaInstructions,
    ArchitecturallyUndefined,
    LoadAndStoreMultiple(LoadAndStoreMultiple),
    BranchAndBranchWithLink(BranchAndBranchWithLink),
    CoprocessorLoadAndStoreAndDoubleRegisterTransfers(CoprocessorLoadAndStoreAndDoubleRegisterTransfers),
    CoprocessorDataProcessing(CoprocessorDataProcessing),
    CoprocessorRegisterTransfers(CoprocessorRegisterTransfers),
    SoftwareInterrupt,
}

impl From<&Instruction> for ArmInstruction {
    fn from(instruction: &Instruction) -> Self {
        match ArmInstructionChecker::from(instruction) {
            ArmInstructionChecker::DataProcessingImmediateShift =>
                Self::DataProcessingImmediateShift(DataProcessingImmediateShift::from(instruction)),
            ArmInstructionChecker::Miscellaneous1 =>
                Self::Miscellaneous1(Miscellaneous1::from(instruction)),
            ArmInstructionChecker::DataProcessingRegisterShift =>
                Self::DataProcessingRegisterShift(DataProcessingRegisterShift::from(instruction)),
            ArmInstructionChecker::Miscellaneous2 =>
                Self::Miscellaneous2(Miscellaneous2::from(instruction)),
            ArmInstructionChecker::Multiplies =>
                Self::Multiplies(Multiplies::from(instruction)),
            ArmInstructionChecker::ExtraLoadAndStores =>
                Self::ExtraLoadAndStores(ExtraLoadAndStores::from(instruction)),
            ArmInstructionChecker::DataProcessingImmediate =>
                Self::DataProcessingImmediate(DataProcessingImmediate::from(instruction)),
            ArmInstructionChecker::UndefinedInstruction =>
                Self::UndefinedInstruction,
            ArmInstructionChecker::MoveImmediateToStatusRegister =>
                Self::MoveImmediateToStatusRegister(MoveImmediateToStatusRegister::from(instruction)),
            ArmInstructionChecker::LoadAndStoreImmediateOffset =>
                Self::LoadAndStoreImmediateOffset(LoadAndStoreImmediateOffset::from(instruction)),
            ArmInstructionChecker::LoadAndStoreRegisterOffset =>
                Self::LoadAndStoreRegisterOffset(LoadAndStoreRegisterOffset::from(instruction)),
            ArmInstructionChecker::MediaInstructions =>
                Self::MediaInstructions,
            ArmInstructionChecker::ArchitecturallyUndefined =>
                Self::ArchitecturallyUndefined,
            ArmInstructionChecker::LoadAndStoreMultiple =>
                Self::LoadAndStoreMultiple(LoadAndStoreMultiple::from(instruction)),
            ArmInstructionChecker::BranchAndBranchWithLink =>
                Self::BranchAndBranchWithLink(BranchAndBranchWithLink::from(instruction)),
            ArmInstructionChecker::CoprocessorLoadAndStoreAndDoubleRegisterTransfers =>
                Self::CoprocessorLoadAndStoreAndDoubleRegisterTransfers(CoprocessorLoadAndStoreAndDoubleRegisterTransfers::from(instruction)),
            ArmInstructionChecker::CoprocessorDataProcessing =>
                Self::CoprocessorDataProcessing(CoprocessorDataProcessing::from(instruction)),
            ArmInstructionChecker::CoprocessorRegisterTransfers =>
                Self::CoprocessorRegisterTransfers(CoprocessorRegisterTransfers::from(instruction)),
            ArmInstructionChecker::SoftwareInterrupt =>
                Self::SoftwareInterrupt,
            ArmInstructionChecker::UnconditionalInstructions =>
                unreachable!("[ArmInstruction Error]: There are no unconditional instructions implemented yet for the current CPUs of the Nintendo DS"),
        }
    }
}
