use core::convert::From;

use crate::cpus::general::{
    instruction::Instruction,
    instruction_map::encoding_types::field::{
        RotateImm,
        Immed8,
    },
    register::types::RegisterIndex,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShifterOperand {
    Immediate {
        rotate_imm: RotateImm, 
        immed_8: Immed8,
    },
    ImmediateShift {
        shift_imm: Immed8,
        shift: Immed8,
        rm: RegisterIndex,
    },
    RegisterShift {
        rs: RegisterIndex,
        shift: Immed8,
        rm: RegisterIndex,
    },
}

impl ShifterOperand {
    pub fn from_immediate(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let rotate_imm = RotateImm::from((instruction_val >> 8) & 0b1111);
        let immed_8 = Immed8::from(instruction_val);

        Self::Immediate {
            rotate_imm,
            immed_8,
        }
    }
    
    pub fn from_shifts(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        if (instruction_val >> 4) & 0b1 == 0b0 {
            let shift_imm = Immed8::from((instruction_val >> 7) & 0b1111);
            let shift = Immed8::from((instruction_val >> 5) & 0b11);
            let rm = RegisterIndex::from(instruction_val & 0b1111);
            
            Self::ImmediateShift {
                shift_imm,
                shift,
                rm,
            }
        } else if (instruction_val >> 4) & 0b1 == 0b1 && (instruction_val >> 7) & 0b1 == 0b0 {
            let rs = RegisterIndex::from((instruction_val >> 8) & 0b1111);
            let shift = Immed8::from((instruction_val >> 5) & 0b11);
            let rm = RegisterIndex::from(instruction_val & 0b1111);

            Self::RegisterShift {
                rs,
                shift,
                rm,
            }
        } else {
            unreachable!("[SHIFTER OPERAND ERROR]: Unknown encoding: {:b}", instruction_val);
        }
    }
}

impl Default for ShifterOperand {
    fn default() -> Self {
        Self::Immediate {
            rotate_imm: RotateImm::from(0),
            immed_8: Immed8::from(0),
        }
    }
}
