use crate::cpus::general::{
    bit_state::BitState, instruction::Instruction,
    instruction_map::instruction_map_trait::InstructionMapTrait,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Branch {
    BOrBL {
        l_flag:          BitState,
        signed_immed_24: u32,
    },
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

    fn match_instruction(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let l_flag = BitState::from((instruction_val >> 24) & 1);
        let signed_immed_24 = instruction_val & 0b111_11111_11111_11111_11111;

        Self::BOrBL {
            l_flag,
            signed_immed_24,
        }
    }
}
