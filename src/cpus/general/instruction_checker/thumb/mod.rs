pub mod error;
pub mod miscellaneous;

use error::ThumbCheckerError;
use miscellaneous::MiscellaneousInstruction;

use crate::cpus::general::{
    Instruction,
    BitState,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThumbInstructionChecker {
    ShiftByImmediate,
    AddSubtractRegister,
    AddSubtractImmediate,
    AddSubtractCompareMoveImmediate,
    DataProcessingRegister,
    SpecialDataProcessing,
    BranchExchangeInstructionSet,
    LoadFromLiteralPool,
    LoadStoreRegisterOffset,
    LoadStoreWordByteImmediateOffset,
    LoadStoreHalfwordImmediateOffset,
    LoadStoreToFromStack,
    AddToSpOrPc,
    Miscellaneous(MiscellaneousInstruction),
    LoadStoreMultiple,
    ConditionalBranch,
    UndefinedInstruction,
    SoftwareInterrupt,
    UnconditionalBranch,
    BlxSuffix,
    BlOrBlxPrefix,
    BlSuffix,
}

impl From<&Instruction> for ThumbInstructionChecker {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let encoding_group = (instruction_val >> 13) & 0b111;

        match encoding_group {
            0b000 => {
                let opcode = (instruction_val >> 11) & 0b11;
                let bit10 = BitState::from(instruction_val >> 10);

                if opcode == 0b11 {
                    if bit10.is_unset() {
                        Self::AddSubtractRegister
                    } else {
                        Self::AddSubtractImmediate
                    }
                } else {
                    Self::ShiftByImmediate
                }
            },
            0b001 => Self::AddSubtractCompareMoveImmediate,
            0b010 => {
                let bit10_12 = (instruction_val >> 10) & 0b111;
                let bit8_9 = (instruction_val >> 8) & 0b11;
                let bit11 = BitState::from(instruction_val >> 11);

                if bit10_12 == 0b000 {
                    Self::DataProcessingRegister
                } else if bit10_12 == 0b001 {
                    if bit8_9 == 0b11 {
                        Self::BranchExchangeInstructionSet
                    } else {
                        Self::SpecialDataProcessing
                    }
                } else if bit11.is_set() {
                    Self::LoadFromLiteralPool
                } else {
                    unreachable!("{}", ThumbCheckerError::UnknownInstructionOfGroup(instruction_val));
                }
            },
            0b011 => Self::LoadStoreHalfwordImmediateOffset,
            0b100 => {
                let bit12 = BitState::from(instruction_val >> 12);

                if bit12.is_unset() {
                    Self::LoadStoreHalfwordImmediateOffset
                } else {
                    Self::LoadStoreToFromStack
                }
            },
            0b101 => {
                let bit12 = BitState::from(instruction_val >> 12);

                if bit12.is_unset() {
                    Self::AddToSpOrPc
                } else {
                    Self::Miscellaneous(MiscellaneousInstruction::from(instruction))
                }
            },
            0b110 => {
                let bit12 = BitState::from(instruction_val >> 12);
                let bit8_11 = (instruction_val >> 8) & 0b1111;

                if bit12.is_unset() {
                    Self::LoadStoreMultiple
                } else {
                    match bit8_11 {
                        0b1110 => Self::UndefinedInstruction,
                        0b1111 => Self::SoftwareInterrupt,
                        _ => Self::ConditionalBranch,
                    }
                }
            },
            0b111 => {
                let bit11_12 = (instruction_val >> 11) & 0b11;
                let bit0 = BitState::from(instruction_val);

                match bit11_12 {
                    0b00 => Self::UnconditionalBranch,
                    0b01 if bit0.is_unset() => Self::BlxSuffix,
                    0b01 if bit0.is_set() => Self::UndefinedInstruction,
                    0b10 => Self::BlOrBlxPrefix,
                    0b11 => Self::BlSuffix,
                    _ => unreachable!("{}", ThumbCheckerError::UnknownInstructionOfGroup(instruction_val)),
                }
            },
            _ => unreachable!("{}", ThumbCheckerError::UnknownInstructionGroup(instruction_val)),
        }
    }
}
