use crate::{
    cpus::general::{
        instruction::decode::DecodeData,
        register::{
            types::ConditionBit,
            NormalizedRegister,
            RegisterName,
        },
        BitState,
    },
    ram::data_types::DataTypeSize,
};

use super::Shift;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShifterOperand {
    pub shifter_operand: u32,
    pub shifter_carry_out: BitState,
}

impl<'a> ShifterOperand {
    pub fn get_immediate_shift(data: DecodeData) -> Self {
        // decode the shifter_operand part
        let rm = {
            let rm = data.instruction.val & 0b1111;
            if NormalizedRegister::from(rm) == NormalizedRegister::from(RegisterName::Pc) {
                let value = data.instruction.address.clone() + DataTypeSize::Byte;
                value.get_as_u32()
            } else {
                rm
            }
        };

        let shift_imm = (data.instruction.val >> 7) & 5;

        let c_flag = {
            let cpsr = data.registers.get_ref_cpsr();
            cpsr.get_condition_bit(ConditionBit::C)
        };

        match Shift::from(data.instruction.val >> 5) {
            Shift::LSL => {
                if shift_imm == 0 {
                    Self {
                        shifter_operand: rm,
                        shifter_carry_out: c_flag,
                    }
                } else {
                    Self {
                        shifter_operand: rm << shift_imm,
                        shifter_carry_out: BitState::from(rm >> (32 - shift_imm)),
                    }
                }
            }
            Shift::LSR => {
                let rs_immed_8 = {
                    let rs = (data.instruction.val >> 8) & 0b1111;
                    data.registers.get_reg(RegisterName::from(rs)) & 0b1111_1111
                };

                if rs_immed_8 == 0 {
                    Self {
                        shifter_operand: rm,
                        shifter_carry_out: c_flag,
                    }
                } else if rs_immed_8 < 32 {
                    Self {
                        shifter_operand: rm << rs_immed_8,
                        shifter_carry_out: BitState::from(rm >> (32 - rs_immed_8)),
                    }
                } else if rs_immed_8 == 32 {
                    Self {
                        shifter_operand: 0,
                        shifter_carry_out: BitState::from(rm),
                    }
                } else {
                    Self {
                        shifter_operand: 0,
                        shifter_carry_out: BitState::Unset,
                    }
                }
            }
            Shift::ASR => {
                if shift_imm == 0 {
                    if (rm >> 31) & 0b1 == 0b0 {
                        Self {
                            shifter_operand: 0,
                            shifter_carry_out: BitState::from(rm >> 31),
                        }
                    } else {
                        Self {
                            shifter_operand: 0xFFFF_FFFF,
                            shifter_carry_out: BitState::from(rm >> 31),
                        }
                    }
                } else {
                    Self {
                        shifter_operand: rm.rotate_right(shift_imm),
                        shifter_carry_out: BitState::from(rm >> (shift_imm - 1)),
                    }
                }
            }
            Shift::ROROrRRX => {
                // it's RRX
                if shift_imm == 0 {
                    Self {
                        shifter_operand: (c_flag.get_as_u32() << 31) | (rm >> 1),
                        shifter_carry_out: BitState::from(rm),
                    }
                }
                // no it's actually ROR
                else {
                    Self {
                        shifter_operand: rm >> shift_imm,
                        shifter_carry_out: BitState::from(rm >> (shift_imm - 1)),
                    }
                }
            }
        }
    }

    pub fn get_immediate(data: DecodeData) -> Self {
        let rotate_imm = (data.instruction.val >> 8) & 0b1111;
        let immed_8 = data.instruction.val & 0b1111_1111;

        let shifter_operand = immed_8.rotate_right(rotate_imm * 2);

        let c_flag = {
            let cpsr = data.registers.get_ref_cpsr();
            cpsr.get_condition_bit(ConditionBit::C)
        };

        if rotate_imm == 0 {
            Self {
                shifter_operand,
                shifter_carry_out: c_flag,
            }
        } else {
            Self {
                shifter_operand,
                shifter_carry_out: BitState::from(shifter_operand >> 31),
            }
        }
    }
}
