use crate::cpus::general::{
    bit_state::BitState, instruction::Instruction,
    instruction_map::instruction_map_trait::InstructionMapTrait,
};

use core::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Branch {
    l_flag: BitState,
    signed_immed_24: i32,
}

impl From<&Instruction> for Branch {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let l_flag = BitState::from((instruction_val >> 24) & 1);
        let signed_immed_24 = i32::try_from(instruction_val & 0b1111_1111_1111_1111_1111_1111)
            .unwrap();

        Self {
            l_flag,
            signed_immed_24,
        }
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
