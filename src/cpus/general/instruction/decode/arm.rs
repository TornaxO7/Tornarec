use crate::cpus::general::instruction::{
    decode::DecodeData,
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
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmDecode {
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

impl<'a> From<DecodeData<'a>> for ArmDecode {
    fn from(decode_data: DecodeData) -> Self {
        match ArmInstructionChecker::from(decode_data.instruction) {
           ArmInstructionChecker::DataProcessingImmediateShift =>
               Self::DataProcessingImmediateShift(DataProcessingImmediateShift::from(decode_data)),
           ArmInstructionChecker::Miscellaneous1 =>
               Self::Miscellaneous1(Miscellaneous1::from(decode_data)),
           ArmInstructionChecker::DataProcessingRegisterShift =>
               Self::DataProcessingRegisterShift(DataProcessingRegisterShift::from(decode_data)),
           ArmInstructionChecker::Miscellaneous2 =>
               Self::Miscellaneous2(Miscellaneous2::from(decode_data)),
           ArmInstructionChecker::Multiplies =>
               Self::Multiplies(Multiplies::from(decode_data)),
           ArmInstructionChecker::ExtraLoadAndStores =>
               Self::ExtraLoadAndStores(ExtraLoadAndStores::from(decode_data)),
           ArmInstructionChecker::DataProcessingImmediate =>
               Self::DataProcessingImmediate(DataProcessingImmediate::from(decode_data)),
           ArmInstructionChecker::UndefinedInstruction =>
               Self::UndefinedInstruction,
           ArmInstructionChecker::MoveImmediateToStatusRegister =>
               Self::MoveImmediateToStatusRegister(MoveImmediateToStatusRegister::from(decode_data)),
           ArmInstructionChecker::LoadAndStoreImmediateOffset =>
               Self::LoadAndStoreImmediateOffset(LoadAndStoreImmediateOffset::from(decode_data)),
           ArmInstructionChecker::LoadAndStoreRegisterOffset =>
               Self::LoadAndStoreRegisterOffset(LoadAndStoreRegisterOffset::from(decode_data)),
           ArmInstructionChecker::MediaInstructions =>
               Self::MediaInstructions,
           ArmInstructionChecker::ArchitecturallyUndefined =>
               Self::ArchitecturallyUndefined,
           ArmInstructionChecker::LoadAndStoreMultiple =>
               Self::LoadAndStoreMultiple(LoadAndStoreMultiple::from(decode_data)),
           ArmInstructionChecker::BranchAndBranchWithLink =>
               Self::BranchAndBranchWithLink(BranchAndBranchWithLink::from(decode_data)),
           ArmInstructionChecker::CoprocessorLoadAndStoreAndDoubleRegisterTransfers =>
               Self::CoprocessorLoadAndStoreAndDoubleRegisterTransfers(CoprocessorLoadAndStoreAndDoubleRegisterTransfers::from(decode_data)),
           ArmInstructionChecker::CoprocessorDataProcessing =>
               Self::CoprocessorDataProcessing(CoprocessorDataProcessing::from(decode_data)),
           ArmInstructionChecker::CoprocessorRegisterTransfers =>
               Self::CoprocessorRegisterTransfers(CoprocessorRegisterTransfers::from(decode_data)),
           ArmInstructionChecker::SoftwareInterrupt =>
               Self::SoftwareInterrupt,
           ArmInstructionChecker::UnconditionalInstructions =>
               unreachable!("[ArmInstruction Error]: There are no unconditional instructions implemented yet for the current CPUs of the Nintendo DS"),
        }
       
    }
}
