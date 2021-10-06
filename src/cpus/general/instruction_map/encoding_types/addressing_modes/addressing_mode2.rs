use crate::cpus::general::instruction::Instruction;

use core::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode2 {
    Immediate(u16),
    Register {
        shift_imm: u8,
        shift: u8,
        rm: u8,
    },
}

impl From<&Instruction> for AddressingMode2 {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        if (instruction_val >> 25) == 0b010 {
            Self::Immediate(u16::try_from(instruction_val & 0b111_1111_1111).unwrap())
        } else {
            let shift_imm = (instruction_val >> 7) & 0b1111;
            let shift = (instruction_val >> 5) & 0b11;
            let rm = instruction_val & 0b1111;

            Self::Register {
                shift_imm,
                shift,
                rm,
            }
        }
    }
}
