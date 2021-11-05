pub mod miscellaneous;

use crate::cpus::general::{
    BitState,
    Instruction,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmInstructionChecker {
    ArchitecturallyUndefined,
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
    MediaInstructions,
    Miscellaneous,
    Multiplies,
    SoftwareInterrupt,
    UnconditionalInstructions,
    UndefinedInstruction,
}

impl From<&Instruction> for ArmInstructionChecker {
    fn from(instruction: &Instruction) -> Self {
        if (instruction.val >> 28) & 0b1111 == 0b1111 {
            return Self::UnconditionalInstructions;
        }

        let opcode = u8::try_from((instruction.val >> 25) & 0b111).unwrap();
        match opcode {
            0b000 => {
                // the most relevant bits to decide which instruction it is, are:
                //  - Bit[4]
                //  - Bit[7]
                //  - Bit[20]
                //  - Bit[24:23]
                let bit4 = BitState::from(instruction.val >> 4);
                let bit7 = BitState::from(instruction.val >> 7);
                let bit20 = BitState::from(instruction.val >> 20);
                let bit24_23 = (instruction.val >> 23) & 0b11;

                if bit4.is_unset() {
                    if bit20.is_unset() && bit24_23 == 0b10 {
                        Self::Miscellaneous
                    } else {
                        Self::DataProcessingImmediateShift
                    }
                } else if bit7.is_unset() && bit20.is_unset() && bit24_23 == 0b10 {
                    Self::Miscellaneous
                } else if bit7.is_set() {
                    let bit4_7 = (instruction.val >> 4) & 0b1111;
                    let bit24_27 = (instruction.val >> 24) & 0b1111;

                    // Differ between Multiplies and ExtraLoadStores
                    if bit4_7 == 0b1001 && bit24_27 == 0b0000 {
                        Self::Multiplies
                    } else {
                        Self::ExtraLoadAndStores
                    }
                } else {
                    Self::DataProcessingRegisterShift
                }
            }
            0b001 => {
                // the relevant bits to differ the different instructions:
                // - Bit[20]
                // - Bit[21]
                // - Bit[24:23]
                let bit20 = BitState::from(instruction.val >> 20);
                let bit21 = BitState::from(instruction.val >> 21);
                let bit24_23 = (instruction.val >> 23) & 0b11;

                if bit24_23 == 0b10 && bit20.is_unset() {
                    match bit21 {
                        BitState::Unset => Self::UndefinedInstruction,
                        _ => todo!("Not implemented yet."),
                    }
                } else {
                    Self::DataProcessingImmediate
                }
            }
            0b010 => Self::LoadAndStoreImmediateOffset,
            0b011 => {
                // the relevant bits which are needed to differ the instructions are:
                // - Bit[4]
                // - Bit[4:7]
                // - Bit[20:24]
                let bit4 = BitState::from(instruction.val >> 4);
                let bit4_7 = (instruction.val >> 4) & 0b1111;
                let bit20_24 = (instruction.val >> 20) & 0b1_1111;

                if bit4.is_unset() {
                    Self::LoadAndStoreRegisterOffset
                } else if bit4_7 == 0b1111 && bit20_24 == 0b1_1111 {
                    Self::ArchitecturallyUndefined
                } else {
                    Self::MediaInstructions
                }
            }
            0b100 => Self::LoadAndStoreMultiple,
            0b101 => Self::BranchAndBranchWithLink,
            0b110 => Self::CoprocessorLoadAndStoreAndDoubleRegisterTransfers,
            0b111 => {
                // the relevant bits which are needed to differ the instructions are:
                // - Bit[24]
                // - Bit[4]
                let bit24 = BitState::from(instruction.val >> 24);
                let bit4 = BitState::from(instruction.val >> 4);

                if bit24.is_set() {
                    Self::SoftwareInterrupt
                } else if bit4.is_set() {
                    Self::CoprocessorRegisterTransfers
                } else {
                    Self::CoprocessorDataProcessing
                }
            }
            _ => unreachable!("The Arm checker should never reach this."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ArmInstructionChecker,
        Instruction,
    };

    #[test]
    fn data_processing_immediate_shift() {
        let instruction = Instruction {
            val: 0b0000_000_1110_0_0000_0000_10101_11_0_1111,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::DataProcessingImmediateShift
        );
    }

    #[test]
    fn miscellaneous1() {
        let instruction = Instruction {
            val: 0b0000_000_1000_0_0000_0000_00000_00_0_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::Miscellaneous,
        );
    }

    #[test]
    fn data_processing_register_shift() {
        let instruction = Instruction {
            val: 0b0000_000_0000_0_0000_0000_0000_0_00_1_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::DataProcessingRegisterShift
        );
    }

    #[test]
    fn miscellaneous2() {
        let instruction = Instruction {
            val: 0b0000_1010_0_0000_0000_0000_0_00_1_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::Miscellaneous
        );
    }

    #[test]
    fn multiplies() {
        let instruction = Instruction {
            val: 0b0000_0000_0000_0000_0000_0000_1001_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::Multiplies
        );
    }

    #[test]
    fn extra_load_stores() {
        let instruction = Instruction {
            val: 0b0000_000_00000_0000_00000_0000_1101_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::ExtraLoadAndStores
        );
    }

    #[test]
    fn data_processing_immediate() {
        let instruction = Instruction {
            val: 0b0000_001_0000_0_0000_0000_0000_0000_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::DataProcessingImmediate
        );
    }

    #[test]
    fn undefined_instruction() {
        let instruction = Instruction {
            val: 0b0000_001_10_0_00_0000_0000_0000_0000_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::UndefinedInstruction
        );
    }

    // #[test]
    // fn move_immediate_to_status_register() {
    //     let instruction = Instruction {
    //         val: 0b0000_001_10_0_10_0000_1111_0000_0000_0000,
    //         ..Instruction::default()
    //     };
    //     assert_eq!(
    //         ArmInstructionChecker::from(&instruction),
    //         ArmInstructionChecker::MoveImmediateToStatusRegister
    //     );
    // }

    #[test]
    fn load_store_immediate_offset() {
        let instruction = Instruction {
            val: 0b0000_010_00000_0000_0000_0000_0000_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::LoadAndStoreImmediateOffset
        );
    }

    #[test]
    fn load_and_store_register_offset() {
        let instruction = Instruction {
            val: 0b0000_011_00000_0000_0000_00000_00_0_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::LoadAndStoreRegisterOffset
        );
    }

    #[test]
    fn media_instructions() {
        let instruction = Instruction {
            val: 0b0000_011_00000_0000_0000_0000_000_1_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::MediaInstructions
        );
    }

    #[test]
    fn architecturally_undefined() {
        let instruction = Instruction {
            val: 0b0000_011_11111_0000_0000_0000_1111_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::ArchitecturallyUndefined
        );
    }

    #[test]
    fn load_and_store_multiple() {
        let instruction = Instruction {
            val: 0b0000_100_00000_0000_0000_0000_0000_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::LoadAndStoreMultiple
        );
    }

    #[test]
    fn branch_and_branch_with_link() {
        let instruction = Instruction {
            val: 0b0000_101_0_0000_0000_0000_0000_0000_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::BranchAndBranchWithLink
        );
    }

    #[test]
    fn coprocessor_load_and_store_and_double_register_transfers() {
        let instruction = Instruction {
            val: 0b0000_110_00000_0000_0000_0000_0000_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::CoprocessorLoadAndStoreAndDoubleRegisterTransfers
        );
    }

    #[test]
    fn coprocessor_data_processing() {
        let instruction = Instruction {
            val: 0b0000_1110_0000_0000_0000_0000_000_0_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::CoprocessorDataProcessing
        );
    }

    #[test]
    fn coprocessor_register_transfers() {
        let instruction = Instruction {
            val: 0b0000_1110_000_0_0000_0000_0000_000_1_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::CoprocessorRegisterTransfers
        );
    }

    #[test]
    fn software_interrupt() {
        let instruction = Instruction {
            val: 0b0000_1111_0000_0000_0000_0000_0000_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::SoftwareInterrupt
        );
    }

    #[test]
    fn unconditional_instructions() {
        let instruction = Instruction {
            val: 0b1111_0000_0000_0000_0000_0000_0000_0000,
            ..Instruction::default()
        };
        assert_eq!(
            ArmInstructionChecker::from(&instruction),
            ArmInstructionChecker::UnconditionalInstructions
        );
    }
}
