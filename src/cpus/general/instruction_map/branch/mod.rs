pub mod error;
pub use error::BranchError;

use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Branch {
    BOrBl {
        l_flag: BitState,
        signed_immed_24: u32,
    },
    Bx {
        rm: u8
    },
}

impl Branch {
    pub fn is_b_or_bl_instruction(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();
        (instruction_val >> 25) & 0b111 == 0b101
    }

    pub fn bx_check_sbo_fields(instruction: &Instruction) {
        let instruction_val = instruction.get_value_as_u32();
        if (instruction_val >> 8) & 0b1111_1111_1111 != 0b1111_1111_1111 {
            panic!("{}", BranchError::SBOConflict(instruction_val));
        }
    }
}

impl From<&Instruction> for Branch {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        if Self::is_b_or_bl_instruction(instruction) {
            Self::BOrBl {
                l_flag: BitState::from((instruction_val >> 24) & 0b1),
                signed_immed_24: instruction_val & 0b1111_1111_1111_1111_1111_1111
            }
        } else {

            Self::bx_check_sbo_fields(instruction);
            Self::Bx {
                rm: instruction_val & 0b1111,
            }
        }
    }
}

#[cfg(test)]
mod test {
    
    use super::Branch;
    use crate::cpus::general::{
        instruction::Instruction,
        bit_state::BitState,
    };

    #[test]
    fn is_b_or_bl_instruction() {
        let b_instruction = Instruction::from(0b0000_1010_0000_0000_0000_0000_0000_0000);
        let bl_instruction = Instruction::from(0b0000_1011_0000_0000_0000_0000_0000_0001);
        let unknown_instruction = Instruction::from(0b0000_1000_0000_0000_0000_0000_0000_0000);

        assert!(Branch::is_b_or_bl_instruction(&b_instruction));
        assert!(Branch::is_b_or_bl_instruction(&bl_instruction));
        assert!(!Branch::is_b_or_bl_instruction(&unknown_instruction));
    }
    
    #[test]
    #[should_panic]
    fn bx_check_invalid_sbo_fields() {
        let bx_instruction = Instruction::from(0b0000_00010010_0101_0010_01110_0001_0110);
        Branch::bx_check_sbo_fields(&bx_instruction);
    }

    #[test]
    fn bx_check_valid_sbo_fields() {
        let bx_instruction = Instruction::from(0b0000_00010010_1111_1111_1111_0001_0110);
        Branch::bx_check_sbo_fields(&bx_instruction);
    }

    #[test]
    fn get_b_or_bl_operand() {
        let b_instruction = Instruction::from(0b0000_1010_0000_0000_0000_0000_0000_0000);
        let bl_instruction = Instruction::from(0b0000_1011_0000_0000_0000_0000_0000_0001);

        let b_branch = Branch::from(&b_instruction);
        let bl_branch = Branch::from(&bl_instruction);

        let b_expected = Branch::BOrBl {
            l_flag: BitState::Unset,
            signed_immed_24: 0,
        };
        
        let bl_expected = Branch::BOrBl {
            l_flag: BitState::Set,
            signed_immed_24: 1,
        };

        assert_eq!(b_branch, b_expected);
        assert_eq!(bl_branch, bl_expected);
    }
}
