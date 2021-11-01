use crate::cpus::general::Instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiscellaneousChecker {
    MoveStatusRegisterToRegister,
    MoveToStatusRegister,
    BranchExchangeInstructionSetThumb,
    BranchExchangeInstructionSetJava,
    CountLeadingZeros,
    BranchAndLinkExchangeInstructionSetThumb,
    SaturatingAddSubtract,
    SoftwareBreakpoint,
    SignedMultiplies,
    Unknown,
}

impl From<&Instruction> for MiscellaneousChecker {
    fn from(instruction: &Instruction) -> Self {
        let instruction23_27 = (instruction.val >> 23) & 0b1_1111;
        let instruction20_21 = (instruction.val >> 20) & 0b11;
        let instruction20_22 = (instruction.val >> 20) & 0b111;

        if instruction23_27 == 0b0_0010 {
            let instruction4_7 = (instruction.val >> 4) & 0b1111;
            match instruction4_7 {
                0b0000 => {
                    match instruction20_21 {
                        0b00 => Self::MoveStatusRegisterToRegister,
                        0b10 => Self::MoveToStatusRegister,
                        _ => Self::Unknown,
                    }
                },
                0b0001 => {
                    match instruction20_22 {
                        0b010 => Self::BranchExchangeInstructionSetThumb,
                        0b110 => Self::CountLeadingZeros,
                        _ => Self::Unknown,
                    }
                },
                0b0010 => Self::BranchExchangeInstructionSetJava,
                0b0011 => Self::BranchAndLinkExchangeInstructionSetThumb,
                0b0101 => Self::SaturatingAddSubtract,
                0b0111 => Self::SoftwareBreakpoint,
                _ => Self::SignedMultiplies,
            }
        } else if instruction23_27 == 0b0_0110 && instruction20_21 == 0b10 {
            Self::MoveToStatusRegister
        } else {
            Self::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    // All tests will have the following format:
    // - The conditionbits are always 0b0000
    // - All other fields, which can be set to 1, are gonna be set to 0, otherwise 0!
    use super::{Instruction, MiscellaneousChecker};

    #[test]
    fn move_status_register_to_register() {
        let instruction = Instruction {
            val: 0b0000_00010_100_1111_1111_0000_0000_0000,
            .. Instruction::default()
        };
        assert_eq!(MiscellaneousChecker::from(&instruction), MiscellaneousChecker::MoveStatusRegisterToRegister);
    }

    #[test]
    fn move_register_to_status_register() {
        let instruction = Instruction {
            val: 0b0000_00010_110_1111_1111_0000_0000_1111,
            .. Instruction::default()
        };
        assert_eq!(MiscellaneousChecker::from(&instruction), MiscellaneousChecker::MoveToStatusRegister);
    }

    #[test]
    fn move_immediate_to_status_register() {
        let instruction = Instruction {
            val: 0b0000_00110_110_1111_1111_1111_1111_1111,
            .. Instruction::default()
        };
        assert_eq!(MiscellaneousChecker::from(&instruction), MiscellaneousChecker::MoveToStatusRegister);
    }

    #[test]
    fn branch_exchange_instruction_set_thumb() {
        let instruction = Instruction {
            val: 0b0000_00010_010_1111_1111_1111_0001_1111,
            .. Instruction::default()
        };
        assert_eq!(MiscellaneousChecker::from(&instruction), MiscellaneousChecker::BranchExchangeInstructionSetThumb);
    }

    #[test]
    fn branch_exchange_instruction_set_java() {
        let instruction = Instruction {
            val: 0b0000_00010_010_1111_1111_1111_0010_1111,
            .. Instruction::default()
        };
        assert_eq!(MiscellaneousChecker::from(&instruction), MiscellaneousChecker::BranchExchangeInstructionSetJava);
    }

    #[test]
    fn count_leading_zeros() {
        let instruction = Instruction {
            val: 0b0000_00010_110_1111_1111_1111_0001_1111,
            .. Instruction::default()
        };
        assert_eq!(MiscellaneousChecker::from(&instruction), MiscellaneousChecker::CountLeadingZeros);
    }

    #[test]
    fn branch_and_link_exchange_instruction_set_thumb() {
        let instruction = Instruction {
            val: 0b0000_00010_010_1111_1111_1111_0011_1111,
            .. Instruction::default()
        };
        assert_eq!(MiscellaneousChecker::from(&instruction), MiscellaneousChecker::BranchAndLinkExchangeInstructionSetThumb);
    }

    #[test]
    fn saturaing_add_subtract() {
        let instruction = Instruction {
            val: 0b0000_00010_110_1111_1111_0000_0101_1111,
            .. Instruction::default()
        };
        assert_eq!(MiscellaneousChecker::from(&instruction), MiscellaneousChecker::SaturatingAddSubtract);
    }

    #[test]
    fn software_breakpoint() {
        let instruction = Instruction {
            val: 0b0000_00010_010_1111_1111_1111_0111_1111,
            .. Instruction::default()
        };
        assert_eq!(MiscellaneousChecker::from(&instruction), MiscellaneousChecker::SoftwareBreakpoint);
    }

    #[test]
    fn signed_multiplies() {
        let instruction = Instruction {
            val: 0b0000_00010_110_1111_1111_1111_1110_1111,
            .. Instruction::default()
        };
        assert_eq!(MiscellaneousChecker::from(&instruction), MiscellaneousChecker::SignedMultiplies);
    }
}
