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
                data.registers.get_reg(NormalizedRegister::from(rm).get_reg())
            }
        };

        let shift_imm = (data.instruction.val >> 7) & 0b1111;

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
                if shift_imm == 0 {
                    Self {
                        shifter_operand: 0,
                        shifter_carry_out: BitState::from(rm >> 31),
                    }
                } else {
                    Self {
                        shifter_operand: rm >> shift_imm,
                        shifter_carry_out: BitState::from(rm >> (shift_imm - 1)),
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

#[cfg(test)]
mod tests {

    use crate::{
        cpus::general::{
            register::{
                types::ConditionBit,
                RegisterName,
            },
            BitState,
            Instruction,
        },
        NintendoDS,
    };

    use super::{
        DecodeData,
        ShifterOperand,
    };

    #[test]
    fn immediate_shift_lsl_if() {
        let mut nds = NintendoDS::default();
        nds.arm7tdmi.registers.set_reg(RegisterName::R1, 1);
        {
            let cpsr = nds.arm7tdmi.registers.get_mut_cpsr();
            cpsr.set_condition_bit(ConditionBit::C, BitState::Set);
        }

        let instruction = Instruction {
            val: 0b0000_000_0000_0_0000_0000_0000_000_0001,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            shifter_operand: 0b1,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(value, expected_value, "{:#?} {:#?}", &value, &expected_value);
    }

    #[test]
    fn immediate_shift_lsl_else() {
        let mut nds = NintendoDS::default();
        nds.arm7tdmi.registers.set_reg(RegisterName::R0, 0b0000_0000_0000_0000__0000_0000_0000_0001);
        let instruction = Instruction {
            val: 0b0000_000_0000_0_0000_0000_0001_000_0000,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            shifter_operand: 0b10,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }

    #[test]
    fn immediate_shift_lsr_if() {
        let mut nds = NintendoDS::default();
        nds.arm7tdmi.registers.set_reg(RegisterName::R0, 0b1000_0000_0000_0000__0000_0000_0000_0000);
        let instruction = Instruction {
            val: 0b0000_000_0000_0_0000_0000_0000_010_0000,
            .. Instruction::default()
        };

        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            shifter_operand: 0,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }

    #[test]
    fn immediate_shift_lsr_else() {
        let mut nds = NintendoDS::default();
        nds.arm7tdmi.registers.set_reg(RegisterName::R2, 0b1000_0000_0000_0000__0000_0000_0000_0000);
        let instruction = Instruction {
            val: 0b0000_000_0000_0_0000_0000_0001_010_0010,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);
        
        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            shifter_operand: 0b0100_0000_0000_0000__0000_0000_0000_0000,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(value, expected_value, "{:#?} {:#?}", &value, &expected_value);
    }

    #[test]
    fn immediate_shift_asr_if_if() {
        let mut nds = NintendoDS::default();
        nds.arm7tdmi.registers.set_reg(RegisterName::R3, 0b0000_0000_0000_0000_0000_0000_0000_0000);
        let instruction = Instruction {
            val: 0b0000_000_0000_0_0000_0000_0000_100_0000,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            shifter_operand: 0,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }

    #[test]
    fn immediate_shift_asr_if_else() {
        let mut nds = NintendoDS::default();
        nds.arm7tdmi.registers.set_reg(RegisterName::R5, 0b1000_0000_0000_0000__0000_0000_0000_0000);
        let instruction = Instruction {
            val: 0b0000_000_0000_0_0000_0000_0000_100_0101,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            shifter_operand: 0xFFFF_FFFF,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(value, expected_value, "{:#?} {:#?}", &value, &expected_value);
    }

    #[test]
    fn immediate_shift_asr_else() {
        let mut nds = NintendoDS::default();
        nds.arm7tdmi.registers.set_reg(RegisterName::R6, 0b0000_0000_0000_0000__0000_0000_0000_0001);
        let instruction = Instruction {
            val: 0b0000_000_0000_0_0000_0000_0001_100_0110,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);
        
        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            shifter_operand: 0b1000_0000_0000_0000__0000_0000_0000_0000,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(value, expected_value, "{:#?} {:#?}", &value, &expected_value);
    }

    #[test]
    fn immediate_shift_rrx() {
        let mut nds = NintendoDS::default();
        {
            let cpsr = nds.arm7tdmi.registers.get_mut_cpsr();
            cpsr.set_condition_bit(ConditionBit::C, BitState::Set);
        }
        nds.arm7tdmi.registers.set_reg(RegisterName::R7, 0b0000_0000_0000_0000__0000_0000_0000_0010);
        let instruction = Instruction {
            val: 0b0000_000_0000_0_0000_0000_0000_110_0111,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            shifter_operand: 0b1000_0000_0000_0000__0000_0000_0000_0001,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }

    #[test]
    fn immediate_shift_ror() {
        let mut nds = NintendoDS::default();
        nds.arm7tdmi.registers.set_reg(RegisterName::R9, 0b0000_0000_0000_0000__0000_0000_0000_0001);
        let instruction = Instruction {
            val: 0b0000_000_0000_0_0000_0000_0001_110_1001,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            shifter_operand: 0b0,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
