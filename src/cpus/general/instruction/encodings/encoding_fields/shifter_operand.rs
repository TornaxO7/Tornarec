use crate::cpus::general::{BitState, instruction::{
        decode::DecodeData,
        encodings::encoding_fields::Shift,
    }, register::{NormalizedRegister, RegisterName, types::ConditionBit}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShifterOperand {
    pub shifter_operand: u32,
    pub shifter_carry_out: BitState,
}

impl<'a> ShifterOperand {
    pub fn get_immediate_shift(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();
        let next_instruction_val = data.next_instruction.get_value_as_u32();

        // decode the shifter_operand part
        let rm = {
            let rm = instruction_val & 0b1111;
            if NormalizedRegister::from(rm) == RegisterName::Pc {
                next_instruction_val
            } else {
                rm
            }
        };

        let rn = {
            let rn = (instruction_val >> 16) & 0b1111;

            if NormalizedRegister::from(rm) == RegisterName::Pc {
                next_instruction_val
            } else {
                rn
            }
        };

        let shift_imm = (instruction_val >> 7) & 5;

        let c_flag = {
            let cpsr = data.registers.get_ref_cpsr();
            cpsr.get_condition_bit(ConditionBit::C)
        };

        match Shift::from(instruction_val >> 5) {
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
            },
            Shift::LSR => {
                let rs_immed_8 = {
                    let rs = (instruction_val >> 8) & 0b1111;
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
            },
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
            },
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
            },
        }
    }

    pub fn get_immediate(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();

        let rotate_imm = (instruction_val >> 8) & 0b1111;
        let immed_8 = instruction_val & 0b1111_1111;

        let shifter_operand = immed_8.rotate_right(rotate_imm * 2);

        let cpsr = data.registers.get_ref_cpsr();
        let c_flag = cpsr.get_condition_bit(ConditionBit::C);

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
