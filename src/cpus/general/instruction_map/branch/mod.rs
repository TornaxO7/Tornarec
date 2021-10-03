pub mod operand;
pub mod error;

pub use operand::BranchOperand;
pub use error::BranchError;

use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
    instruction_map::instruction_map_trait::InstructionMapTrait,
};

use core::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Branch(Instruction);

impl From<&Instruction> for Branch {
    fn from(instruction: &Instruction) -> Self {
        Self(instruction.clone())
    }
}

impl InstructionMapTrait for Branch {

    type Operand = BranchOperand;

    fn is_matching(instruction: &Instruction) -> bool {

        if (BranchOperand::is_b_or_bl_instruction(instruction)) ||
            (BranchOperand::is_bx_instruction(instruction))
        {
            true
        } else {
            false
        }
    }

    fn get_operand(&self) -> Self::Operand {
        let instruction_val = self.0.get_value_as_u32();

        if BranchOperand::is_bx_instruction(&self.0) {
            if (instruction_val >> 8) & 0b1111_1111_1111 != 0b1111_1111_1111 {
                panic!("{}", BranchError::SBOConflict(instruction_val));
            }

            let rm = u8::try_from(instruction_val & 0b1111).unwrap();
            let switch_to_thumb = BitState::from(instruction_val & 0b1);

            BranchOperand::Bx {
                rm,
                switch_to_thumb,
            }
        } else {
            let l = BitState::from((instruction_val >> 24) & 0b1);
            let signed_immed_24 = i32::try_from(instruction_val & 0b1111_1111_1111_1111_1111_1111).unwrap();

            BranchOperand::BOrBL {
                l,
                signed_immed_24,
            }
        }
    }
}

#[cfg(test)]
mod test {
    
    use super::{Branch, BranchOperand};
    use crate::cpus::general::{
        instruction::Instruction,
        instruction_map::InstructionMapTrait,
        bit_state::BitState,
    };

    #[test]
    fn is_matching() {
        let valid_instruction = Instruction::from(0b0000_1010_0000_0000_0000_0000_0000_0000);
        let invalid_instruction = Instruction::from(0b0000_0100_0000_0000_0000_0000_0000_0000);

        assert!(Branch::is_matching(&valid_instruction));
        assert!(!Branch::is_matching(&invalid_instruction));
    }

    #[test]
    fn get_bx_operand() {
        let bx_instruction = Instruction::from(0b0000_00010010_1111_1111_1111_0001_0000);
        let bx_branch = Branch::from(&bx_instruction);
        let bx_operand = bx_branch.get_operand();

        assert_eq!(bx_operand, BranchOperand::Bx {
            rm: 0b0000,
            switch_to_thumb: BitState::Unset,
        });
    }

    #[test]
    #[should_panic]
    fn get_invalid_bx_operand() {
        let invalid_bx_instruction = Instruction::from(0b0000_00010010_0000_0000_0000_0001_0000);
        let invalid_bx_branch = Branch::from(&invalid_bx_instruction);

        invalid_bx_branch.get_operand();
    }

    #[test]
    fn get_b_or_bl_operand() {
        let b_instruction = Instruction::from(0b0000_1010_0000_0000_0000_0000_0000_0000);
        let bl_instruction = Instruction::from(0b0000_1011_0000_0000_0000_0000_0000_0001);

        let b_branch = Branch::from(&b_instruction);
        let bl_branch = Branch::from(&bl_instruction);

        let b_operand = b_branch.get_operand();
        let bl_operand = bl_branch.get_operand();

        let expected_b_operand = BranchOperand::BOrBL {
            l: BitState::Unset,
            signed_immed_24: 0,
        };

        let expected_bl_operand = BranchOperand::BOrBL {
            l: BitState::Set,
            signed_immed_24: 1,
        };

        assert_eq!(b_operand, expected_b_operand);
        assert_eq!(bl_operand, expected_bl_operand);
    }
}
