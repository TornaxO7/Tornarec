pub mod error;

use error::ArmCheckerError;
use crate::cpus::general::{Instruction, BitState};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmInstructionChecker {
    DataProcessingImmediateShift,
    Miscellaneous1,
    DataProcessingRegisterShift,
    Miscellaneous2,
    Multiplies,
    ExtraLoadStores,
    DataProcessingImmediate,
    UndefinedInstruction,
    MoveImmediateToCpsr,
    LoadStoreImmediateOffset,
    LoadStoreRegisterOffset,
    MediaInstructions,
    ArchitecturallyUndefined,
    LoadStoreMultiple,
    BranchAndBranchWithLink,
    CoprocessorLoadStoreAndDoubleRegisterTransfers,
    CoprocessorDataProcessing,
    CoprocessorRegisterTransfers,
    SoftwareInterrupt,
    UnconditionalInstruction,
}

impl From<&Instruction> for ArmInstructionChecker {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 25) & 0b111).unwrap();
        match opcode {
            0b000 => {
                // the most relevant bits to decide which instruction it is, are:
                //  - Bit[4]
                //  - Bit[7]
                //  - Bit[20]
                //  - Bit[24:23]
                let bit4 = BitState::from(instruction_val >> 4);
                let bit7 = BitState::from(instruction_val >> 7);
                let bit20 = BitState::from(instruction_val >> 20);
                let bit24_23_is_10 = (instruction_val >> 23) & 0b11 == 0b10;

                if bit4.is_unset() {
                    if bit20.is_unset() && bit24_23_is_10 {
                        Self::Miscellaneous1
                    } else {
                        Self::DataProcessingImmediateShift
                    }
                } else {
                    if bit7.is_unset() && bit20.is_unset() && bit24_23_is_10 {
                        Self::Miscellaneous2
                    } else if bit7.is_set() {
                        // Differ between Multiplies and ExtraLoadStores
                        if (instruction_val >> 4) & 0b1111 == 0b1001 {
                            Self::Multiplies
                        } else {
                            Self::ExtraLoadStores
                        }
                    } else {
                        Self::DataProcessingRegisterShift
                    }
                }
            },
            0b001 => {
                // the relevant bits to differ the different instructions:
                // - Bit[20]
                // - Bit[21]
                // - Bit[24:23]
                let bit20 = BitState::from(instruction_val >> 20);
                let bit21 = BitState::from(instruction_val >> 21);
                let bit24_23 = (instruction_val >> 23) & 0b11;

                if bit24_23 == 0b10 && bit20.is_unset() {
                    match bit21 {
                        BitState::Unset => Self::UndefinedInstruction,
                        BitState::Set => Self::MoveImmediateToCpsr,
                        _other => unreachable!("{}", ArmCheckerError::UnknownInstruction(instruction_val)),
                    }
                } else {
                    Self::DataProcessingImmediate
                }
            },
            0b010 => Self::LoadStoreImmediateOffset,
            0b011 => {
                // the relevant bits which are needed to differ the instructions are:
                // - Bit[4]
                // - Bit[4:7]
                // - Bit[20:24]
                let bit4 = BitState::from(instruction_val >> 4);
                let bit4_7 = (instruction_val >> 4) & 0b1111;
                let bit20_24 = (instruction_val >> 20) & 0b1_1111;

                if bit4.is_unset() {
                    Self::LoadStoreRegisterOffset
                } else if bit4_7 == 0b1111 && bit20_24 == 0b1_1111 {
                    Self::ArchitecturallyUndefined
                } else {
                    Self::MediaInstructions
                }

            },
            0b100 => Self::LoadStoreMultiple,
            0b101 => Self::BranchAndBranchWithLink,
            0b110 => Self::CoprocessorLoadStoreAndDoubleRegisterTransfers,
            0b111 => {
                // the relevant bits which are needed to differ the instructions are:
                // - Bit[24]
                // - Bit[4]
                let bit24 = BitState::from(instruction_val >> 24);
                let bit4 = BitState::from(instruction_val >> 4);

                if bit24.is_set() {
                    Self::SoftwareInterrupt
                } else if bit4.is_set() {
                    Self::CoprocessorRegisterTransfers
                } else {
                    Self::CoprocessorDataProcessing
                }
            },
            _ => unreachable!("The Arm checker should never reach this."),
        }
    }
}
