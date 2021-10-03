use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchOperand {
    BOrBL {
        l: BitState,
        signed_immed_24: i32
    },
    Bx {
        rm: u8,
        switch_to_thumb: BitState,
    },
}

impl BranchOperand {
    pub fn is_b_or_bl_instruction(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();

        (instruction_val >> 25) & 0b111 == 0b101
    }

    pub fn is_bx_instruction(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();

        (instruction_val >> 20) & 0b1111_1111 == 0b0001_0010
            && (instruction_val >> 4) & 0b1111 == 0b0001
    }
}
