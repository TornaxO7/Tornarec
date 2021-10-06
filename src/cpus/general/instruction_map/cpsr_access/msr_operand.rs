use crate::cpus::general::{
    BitState,
    instruction_map::{
        cpsr_access::CpsrAccessError,
        encoding_types::field::FieldMask,
    },
    instruction::Instruction,
};

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MsrOperand {
    Immediate {
        r_flag: BitState,
        field_mask: FieldMask,
        rotate_imm: u8,
        immed8: u8,
    },
    Register {
        r_flag: BitState,
        field_mask: FieldMask,
        rm: u8,
    }
}

impl MsrOperand {
    fn is_immediate_operand(instruction: &Instruction) -> bool {
        (instruction.get_value_as_u32() >> 25) & 0b1 == 0b1
    }
}

impl From<&Instruction> for MsrOperand {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let r_flag = BitState::from((instruction_val >> 22) & 0b1);
        let field_mask = FieldMask::from((instruction_val >> 16) & 0b1111);

        if MsrOperand::is_immediate_operand(instruction) {
            let rotate_imm = (instruction_val >> 8) & 0b1111;
            let immed8 = instruction_val & 0b1111_1111;

            if (instruction_val >> 12) & 0b1111 != 0b1111 {
                panic!("{}", CpsrAccessError::SBOConflict(16, 19, instruction_val));
            }

            Self::Immediate {
                r_flag,
                field_mask,
                rotate_imm,
                immed8,
            }

        } else {
            let rm = instruction_val & 0b1111;

            if (instruction_val >> 12) & 0b1111 != 0b1111 {
                panic!("{}", CpsrAccessError::SBOConflict(16, 19, instruction_val));
            }
            if (instruction_val >> 8) & 0b1111 != 0b0000 {
                panic!("{}", CpsrAccessError::SBZConflict(8, 11, instruction_val));
            }

            Self::Register {
                r_flag,
                field_mask,
                rm,
            }
        }
    }
}
