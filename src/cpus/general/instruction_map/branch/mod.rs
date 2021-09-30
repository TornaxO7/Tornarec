pub mod operand;

pub use operand::BranchOperand;

use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
    instruction_map::instruction_map_trait::InstructionMapTrait,
};

use core::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Branch(Instruction);

impl Branch {
    pub fn get_operand(&self) -> BranchOperand {
        let instruction_val = self.0.get_value_as_u32();

        if ((instruction_val >> 20) & 0b111_1111 == 0b00010010)
            && ((instruction_val >> 4) & 0b1111 == 0b1111) 
        {
            if (instruction_val >> 8) & 0b1111_1111_1111 != 0b1111_1111_1111 {
                panic!("[BRANCH ERROR]: Bit[8:19] should be ones! Instruction value: {:b}", instruction_val);
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

impl From<&Instruction> for Branch {
    fn from(instruction: &Instruction) -> Self {
        Self(instruction.clone())
    }
}

impl InstructionMapTrait for Branch {
    fn is_matching(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();

        if (instruction_val >> 25) & 0b111 == 0b101 {
            true
        } else {
            false
        }
    }
}
