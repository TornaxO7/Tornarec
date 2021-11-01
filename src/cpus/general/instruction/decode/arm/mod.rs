mod error;

pub use error::ArmDecodeError;

use crate::cpus::general::instruction::{
    checker::ArmInstructionChecker,
    decode::DecodeData,
    encodings::arm::{
        BranchAndBranchWithLink,
        CoprocessorDataProcessing,
        CoprocessorLoadAndStoreAndDoubleRegisterTransfers,
        CoprocessorRegisterTransfers,
        DataProcessingImmediate,
        DataProcessingImmediateShift,
        DataProcessingRegisterShift,
        ExtraLoadAndStores,
        LoadAndStoreImmediateOffset,
        LoadAndStoreMultiple,
        LoadAndStoreRegisterOffset,
        Multiplies,
    },
};

use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmDecode {
    DataProcessingImmediateShift(DataProcessingImmediateShift),
    DataProcessingRegisterShift(DataProcessingRegisterShift),
    Multiplies(Multiplies),
    ExtraLoadAndStores(ExtraLoadAndStores),
    DataProcessingImmediate(DataProcessingImmediate),
    UndefinedInstruction,
    LoadAndStoreImmediateOffset(LoadAndStoreImmediateOffset),
    LoadAndStoreRegisterOffset(LoadAndStoreRegisterOffset),
    MediaInstructions,
    ArchitecturallyUndefined,
    LoadAndStoreMultiple(LoadAndStoreMultiple),
    BranchAndBranchWithLink(BranchAndBranchWithLink),
    CoprocessorLoadAndStoreAndDoubleRegisterTransfers(
        CoprocessorLoadAndStoreAndDoubleRegisterTransfers,
    ),
    CoprocessorDataProcessing(CoprocessorDataProcessing),
    CoprocessorRegisterTransfers(CoprocessorRegisterTransfers),
    SoftwareInterrupt,
}

impl<'a> TryFrom<DecodeData<'a>> for ArmDecode {
    type Error = ArmDecodeError;

    fn try_from(decode_data: DecodeData<'a>) -> Result<Self, Self::Error> {
        match ArmInstructionChecker::from(&decode_data.instruction) {
           ArmInstructionChecker::DataProcessingImmediateShift =>
               Ok(Self::DataProcessingImmediateShift(DataProcessingImmediateShift::from(decode_data))),
           ArmInstructionChecker::DataProcessingRegisterShift =>
               Ok(Self::DataProcessingRegisterShift(DataProcessingRegisterShift::from(decode_data))),
           ArmInstructionChecker::Multiplies =>
               Ok(Self::Multiplies(Multiplies::from(decode_data))),
           ArmInstructionChecker::ExtraLoadAndStores =>
               Ok(Self::ExtraLoadAndStores(ExtraLoadAndStores::from(decode_data))),
           ArmInstructionChecker::DataProcessingImmediate =>
               Ok(Self::DataProcessingImmediate(DataProcessingImmediate::from(decode_data))),
           ArmInstructionChecker::UndefinedInstruction =>
               Ok(Self::UndefinedInstruction),
           ArmInstructionChecker::LoadAndStoreImmediateOffset =>
               Ok(Self::LoadAndStoreImmediateOffset(LoadAndStoreImmediateOffset::from(decode_data))),
           ArmInstructionChecker::LoadAndStoreRegisterOffset =>
               Ok(Self::LoadAndStoreRegisterOffset(LoadAndStoreRegisterOffset::from(decode_data))),
           ArmInstructionChecker::MediaInstructions =>
               Ok(Self::MediaInstructions),
           ArmInstructionChecker::ArchitecturallyUndefined =>
               Ok(Self::ArchitecturallyUndefined),
           ArmInstructionChecker::LoadAndStoreMultiple =>
               Ok(Self::LoadAndStoreMultiple(LoadAndStoreMultiple::from(decode_data))),
           ArmInstructionChecker::BranchAndBranchWithLink =>
               Ok(Self::BranchAndBranchWithLink(BranchAndBranchWithLink::from(decode_data))),
           ArmInstructionChecker::CoprocessorLoadAndStoreAndDoubleRegisterTransfers =>
               Ok(Self::CoprocessorLoadAndStoreAndDoubleRegisterTransfers(CoprocessorLoadAndStoreAndDoubleRegisterTransfers::from(decode_data))),
           ArmInstructionChecker::CoprocessorDataProcessing =>
               Ok(Self::CoprocessorDataProcessing(CoprocessorDataProcessing::from(decode_data))),
           ArmInstructionChecker::CoprocessorRegisterTransfers =>
               Ok(Self::CoprocessorRegisterTransfers(CoprocessorRegisterTransfers::from(decode_data))),
           ArmInstructionChecker::SoftwareInterrupt =>
               Ok(Self::SoftwareInterrupt),
           ArmInstructionChecker::UnconditionalInstructions =>
               unreachable!("[ArmInstruction Error]: There are no unconditional instructions implemented yet for the current CPUs of the Nintendo DS"),
        }
    }
}
