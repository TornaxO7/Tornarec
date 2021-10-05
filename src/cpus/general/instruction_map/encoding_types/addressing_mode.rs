use crate::cpus::general::{
    instruction::Instruction,
    bit_state::BitState,
    register::types::RegisterIndex,
    instruction_map::encoding_types::field::Immed8,
};

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate(u16),
    Register {
        shift_imm: Immed8,
        shift: Immed8,
        rm: RegisterIndex,
    },
}

impl From<&Instruction> for AddressingMode {
    fn from(instruction: &Instruction) -> Self {
        let instructionv_val = instruction.get_value_as_u32();

        if (instruction_val >> 25) == 0b010 {
            Self::Immediate(instruction_val & 0b111_1111_1111)
        } else {
            let shift_imm = Immed8::from((instruction_val >> 7) & 0b1111);
            let shift = Immed8::from((instruction_val >> 5) & 0b11);
            let rm = RegisterIndex::from(instruction_val & 0b1111);

            Self::Register {
                shift_imm,
                shift,
                rm,
            }
        }
    }
}
