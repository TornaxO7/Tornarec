use crate::cpus::general::instruction::Instruction;

use core::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate(u16),
    Register {
        shift_imm: u8,
        shift: u8,
        rm: u8,
    },
}

impl AddressingMode {
    fn is_immediate_offset(instruction: &Instruction) -> bool {
        (instruction.get_valu_as_u32() >> 25) & 0b111 == 0b010
    }
}

impl From<&Instruction> for AddressingMode {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        if Self::is_immediate_offset(instruction) {
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
