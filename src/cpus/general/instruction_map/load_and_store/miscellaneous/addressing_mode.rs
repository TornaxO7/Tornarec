use crate::cpus::general::instruction::Instruction;

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate {
        immed_h: u8,
        immed_l: u8,
    },
    Register {
        rm: u8
    },
}

impl From<&Instruction> for AddressingMode {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let immed_h = (instruction_val >> 8) & 0b1111;
        let sbz = immed_h;

        let immed_l = instruction_val & 0b1111;
        let rm = immed_l;

        if (instruction_val >> 22) & 0b1 == 1 {
            Self::Immediate{immed_h, immed_l}
        } else {
            if sbz != 0b0000 {
                panic!("[MISCELLANEOUS LOADS AND STORES ERROR]: SBZ isn't set to zero: {:b}", instruction_val);
            }
            Self::Register{rm}
        }
    }
}
