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
        let encoding_group = (instruction.val >> 13) & 0b111;

        match encoding_group {
            0b000 => {
                let opcode = (instruction.val >> 11) & 0b11;
                let bit10 = BitState::from(instruction.val >> 10);

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
                let bit8_9 = (instruction.val >> 8) & 0b11;
                let bit10 = BitState::from(instruction.val >> 10);
                let bit11 = BitState::from(instruction.val >> 11);
                let bit12 = BitState::from(instruction.val >> 12);

                if bit12.is_set() {
                    Self::LoadStoreRegisterOffset
                } else if bit11.is_set() {
                    Self::LoadFromLiteralPool
                } else if bit10.is_unset() {
                    Self::DataProcessingRegister
                } else if bit8_9 == 0b11 {
                    Self::BranchExchangeInstructionSet
                } else {
                    Self::SpecialDataProcessing
                }
            },
            0b011 => Self::LoadStoreWordByteImmediateOffset,
            0b100 => {
                let bit12 = BitState::from(instruction.val >> 12);

                if bit12.is_unset() {
                    Self::LoadStoreHalfwordImmediateOffset
                } else {
                    Self::LoadStoreToFromStack
                }
            },
            0b101 => {
                let bit12 = BitState::from(instruction.val >> 12);

                if bit12.is_unset() {
                    Self::AddToSpOrPc
                } else {
                    Self::Miscellaneous(MiscellaneousInstruction::from(instruction))
                }
            },
            0b110 => {
                let bit12 = BitState::from(instruction.val >> 12);
                let bit8_11 = (instruction.val >> 8) & 0b1111;

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
                let bit11_12 = (instruction.val >> 11) & 0b11;
                let bit0 = BitState::from(instruction.val);

                match bit11_12 {
                    0b00 => Self::UnconditionalBranch,
                    0b01 if bit0.is_unset() => Self::BlxSuffix,
                    0b01 if bit0.is_set() => Self::UndefinedInstruction,
                    0b10 => Self::BlOrBlxPrefix,
                    0b11 => Self::BlSuffix,
                    _ => unreachable!("{}", ThumbCheckerError::UnknownInstructionOfGroup(instruction.val)),
                }
            },
            _ => unreachable!("{}", ThumbCheckerError::UnknownInstructionGroup(instruction.val)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ThumbInstructionChecker, Instruction, MiscellaneousInstruction};

    #[test]
    fn shift_by_immediate() {
        let instruction = Instruction {
            val: 0b000_00_00000_000_000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::ShiftByImmediate);
    }

    #[test]
    fn add_subtract_register() {
        let instruction = Instruction {
            val: 0b000_11_0_0_000_000_000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::AddSubtractRegister);
    }

    #[test]
    fn add_subtract_immediate() {
        let instruction = Instruction {
            val: 0b000_11_1_0_000_000_000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::AddSubtractImmediate);
    }

    #[test]
    fn add_subtract_compare_move_immediate() {
        let instruction = Instruction {
            val: 0b001_00_000_0000_0000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::AddSubtractCompareMoveImmediate);
    }

    #[test]
    fn data_processing_register() {
        let instruction = Instruction {
            val: 0b010000_0000_000_000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::DataProcessingRegister);
    }

    #[test]
    fn special_data_processing() {
        let instruction = Instruction {
            val: 0b010001_00_0_0000_000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::SpecialDataProcessing);
    }

    #[test]
    fn branch_exchange_instruction_set() {
        let instruction = Instruction {
            val: 0b010001_11_0_0_000_000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::BranchExchangeInstructionSet);
    }

    #[test]
    fn load_from_literal_pool() {
        let instruction = Instruction {
            val: 0b01001_000_0000_0000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::LoadFromLiteralPool);
    }

    #[test]
    fn load_store_register_offset() {
        let instruction = Instruction {
            val: 0b0101_000_000_000_000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::LoadStoreRegisterOffset);
    }

    #[test]
    fn load_store_word_byte_immediate_offset() {
        let instruction = Instruction {
            val: 0b011_0_0_00000_000_000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::LoadStoreWordByteImmediateOffset);
    }

    #[test]
    fn load_store_halfword_immediate_offset() {
        let instruction = Instruction {
            val: 0b1000_0_00000_000_000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::LoadStoreHalfwordImmediateOffset);
    }

    #[test]
    fn load_store_to_from_stack() {
        let instruction = Instruction {
            val: 0b1001_0_000_0000_0000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::LoadStoreToFromStack);
    }

    #[test]
    fn add_to_sp_or_pc() {
        let instruction = Instruction {
            val: 0b1010_0_000_0000_0000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::AddToSpOrPc);
    }

    #[test]
    fn miscellaneous_adjust_stack_pointer() {
        let instruction = Instruction {
            val: 0b1011_0000_0_000_0000,
            .. Instruction::default()
        };
        assert_eq!(
            ThumbInstructionChecker::from(&instruction),
            ThumbInstructionChecker::Miscellaneous(MiscellaneousInstruction::AdjustStackPointer)
        );
    }

    #[test]
    fn miscellaneous_push_pop_register_list() {
        let instruction = Instruction {
            val: 0b1011_0100_0000_0000,
            .. Instruction::default()
        };
        assert_eq!(
            ThumbInstructionChecker::from(&instruction),
            ThumbInstructionChecker::Miscellaneous(MiscellaneousInstruction::PushPopRegisterList),
        );
    }

    #[test]
    fn miscellaneous_software_breakpoint() {
        let instruction = Instruction {
            val: 0b1011_1110_0000_0000,
            .. Instruction::default()
        };
        assert_eq!(
            ThumbInstructionChecker::from(&instruction),
            ThumbInstructionChecker::Miscellaneous(MiscellaneousInstruction::SoftwareBreakpoint),
        );
    }

    #[test]
    fn load_store_multiple() {
        let instruction = Instruction {
            val: 0b1100_0_000_0000_0000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::LoadStoreMultiple);
    }

    #[test]
    fn conditional_branch() {
        let instruction = Instruction {
            val: 0b1101_0000_0000_0000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::ConditionalBranch);
    }

    #[test]
    fn undefined_instruction2() {
        let instruction = Instruction {
            val: 0b1101_1110_0000_0000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::UndefinedInstruction);
    }

    #[test]
    fn software_interrupt() {
        let instruction = Instruction {
            val: 0b1101_1111_0000_0000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::SoftwareInterrupt);
    }

    #[test]
    fn unconditional_branch() {
        let instruction = Instruction {
            val: 0b11100_0_00000_00000,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::UnconditionalBranch);
    }

    #[test]
    fn blx_suffix() {
        let instruction = Instruction {
            val: 0b11101_00000_00000_0,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::BlxSuffix);
    }

    #[test]
    fn undefined_instruction1() {
        let instruction = Instruction {
            val: 0b11101_00000_00000_1,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::UndefinedInstruction);
    }

    #[test]
    fn bl_blx_prefix() {
        let instruction = Instruction {
            val: 0b11110_00000_00000_0,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::BlOrBlxPrefix);
    }

    #[test]
    fn bl_suffix() {
        let instruction = Instruction {
            val: 0b11111_00000_00000_0,
            .. Instruction::default()
        };
        assert_eq!(ThumbInstructionChecker::from(&instruction), ThumbInstructionChecker::BlSuffix);
    }
}
