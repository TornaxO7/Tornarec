use crate::cpus::general::{
    instruction::{
        decode::DecodeData,
        encodings::encoding_fields::Shift,
    },
    register::{
        NormalizedRegister,
        RegisterName,
    },
    BitState,
};

use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShifterOperand {
    pub shifter_operand: u32,
    pub shifter_carry_out: Option<BitState>,
}

impl<'a> ShifterOperand {
    pub fn get_immediate_shift(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();
        let next_instruction_val = data.next_instruction.get_value_as_u32();

        // decode the shifter_operand part
        let mut rm = instruction_val & 0b1111;
        let mut rn = (instruction_val >> 16) & 0b1111;

        if NormalizedRegister::from(rm) == RegisterName::Pc {
            rm = next_instruction_val;
        }

        if NormalizedRegister::from(rn) == RegisterName::Pc {
            rn = next_instruction_val;
        }

        let shift_imm = (instruction_val >> 7) & 5;

        // TODO: HERE
        match Shift::from(instruction_val >> 5) {
        };

        if shift_imm == 0 {
            Self {
                shifter_operand: rm,
                shifter_carry_out: None,
            }
        } else {
            
            let shifter_operand = match Shift::from(instruction_val >> 5) {
                Shift::Left => rm << shift_imm,
                Shift::Right => rm >> shift_imm,
            }

            Self {
                shifter_operand: rm << shift_imm,
                shifter_carry_out: Some(BitState::from(rm >> shift_imm)),
            }
        }
    }

    pub fn get_immediate(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();

        let rotate_imm = (instruction_val >> 8) & 0b1111;
        let immed_8 = instruction_val & 0b1111_1111;

        let shifter_operand = immed_8.rotate_right(rotate_imm * 2);

        if rotate_imm == 0 {
            Self {
                shifter_operand,
                shifter_carry_out: None,
            }
        } else {
            Self {
                shifter_operand,
                shifter_carry_out: Some(BitState::from(shifter_operand >> 31)),
            }
        }
    }
}
