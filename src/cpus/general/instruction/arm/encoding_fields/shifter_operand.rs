use crate::{
    cpus::general::{
        instruction::decode::DecodeData,
        register::{
            types::ConditionBit,
            RegisterName,
        },
        BitState,
    },
    ram::data_types::DataTypeSize,
};

use super::Shift;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShifterOperand {
    /// The actual value of the shifter operand
    pub val: u32,
    pub shifter_carry_out: BitState,
}

impl<'a> ShifterOperand {
    pub fn get_immediate(data: DecodeData) -> Self {
        Self::get_immediate_shift(data)
    }

    pub fn get_immediate_shift(data: DecodeData) -> Self {
        // decode the shifter_operand part
        let rm = {
            let rm = data.instruction.val & 0b1111;
            if RegisterName::from(rm) == RegisterName::Pc {
                let value = data.instruction.address + DataTypeSize::Byte;
                value.get_as_u32()
            } else {
                data.registers.get_reg(RegisterName::from(rm))
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
                        val: rm,
                        shifter_carry_out: c_flag,
                    }
                } else {
                    Self {
                        val: rm << shift_imm,
                        shifter_carry_out: BitState::from(rm >> (32 - shift_imm)),
                    }
                }
            }
            Shift::LSR => {
                if shift_imm == 0 {
                    Self {
                        val: 0,
                        shifter_carry_out: BitState::from(rm >> 31),
                    }
                } else {
                    Self {
                        val: rm >> shift_imm,
                        shifter_carry_out: BitState::from(rm >> (shift_imm - 1)),
                    }
                }
            }
            Shift::ASR => {
                if shift_imm == 0 {
                    if BitState::from(rm >> 31) == BitState::Unset {
                        Self {
                            val: 0,
                            shifter_carry_out: BitState::Unset,
                        }
                    } else {
                        Self {
                            val: 0xFFFF_FFFF,
                            shifter_carry_out: BitState::Set,
                        }
                    }
                } else {
                    Self {
                        val: rm.rotate_right(shift_imm),
                        shifter_carry_out: BitState::from(rm >> (shift_imm - 1)),
                    }
                }
            }
            Shift::ROROrRRX => {
                // it's RRX
                if shift_imm == 0 {
                    Self {
                        val: (c_flag.get_as_u32() << 31) | (rm >> 1),
                        shifter_carry_out: BitState::from(rm),
                    }
                }
                // no it's actually ROR
                else {
                    Self {
                        val: rm >> shift_imm,
                        shifter_carry_out: BitState::from(rm >> (shift_imm - 1)),
                    }
                }
            }
        }
    }

    pub fn get_register_shift(data: DecodeData) -> Self {
        // UNPREDICTABLE: Handle it!
        let rs_reg = RegisterName::from(data.instruction.val >> 8);
        let rm_reg = RegisterName::from(data.instruction.val);

        let rs_val = data.registers.get_reg(rs_reg);
        let rm_val = data.registers.get_reg(rm_reg);

        let rs_immed_8 = rs_val & 0b1111_1111;

        let c_flag = {
            let cpsr = data.registers.get_ref_cpsr();
            cpsr.get_condition_bit(ConditionBit::C)
        };

        match Shift::from(data.instruction.val >> 5) {
            Shift::LSL => {
                if rs_immed_8 == 0 {
                    Self {
                        val: rm_val,
                        shifter_carry_out: c_flag,
                    }
                } else if rs_immed_8 < 32 {
                    Self {
                        val: rm_val << rs_immed_8,
                        shifter_carry_out: BitState::from(rm_val >> (32 - rs_immed_8)),
                    }
                } else if rs_immed_8 == 32 {
                    Self {
                        val: 0,
                        shifter_carry_out: BitState::from(rm_val),
                    }
                } else {
                    Self {
                        val: 0,
                        shifter_carry_out: BitState::Unset,
                    }
                }
            }
            Shift::ASR => {
                if rs_immed_8 == 0 {
                    Self {
                        val: rm_val,
                        shifter_carry_out: c_flag,
                    }
                } else if rs_immed_8 < 32 {
                    Self {
                        val: rm_val >> rs_immed_8,
                        shifter_carry_out: BitState::from(rm_val >> (rs_immed_8 - 1)),
                    }
                } else {
                    match BitState::from(rm_val >> 31) {
                        BitState::Unset => Self {
                            val: 0,
                            shifter_carry_out: BitState::Unset,
                        },
                        BitState::Set => Self {
                            val: 0xFFFF_FFFF,
                            shifter_carry_out: BitState::from(rm_val >> 31),
                        },
                    }
                }
            }
            Shift::LSR => {
                if rs_immed_8 == 0 {
                    Self {
                        val: rm_val,
                        shifter_carry_out: c_flag,
                    }
                } else if rs_immed_8 < 32 {
                    Self {
                        val: rm_val >> rs_immed_8,
                        shifter_carry_out: BitState::from(rm_val >> (rs_immed_8 - 1)),
                    }
                } else if rs_immed_8 == 32 {
                    Self {
                        val: 0,
                        shifter_carry_out: BitState::from(rm_val >> 31),
                    }
                } else {
                    Self {
                        val: 0,
                        shifter_carry_out: BitState::Unset,
                    }
                }
            }
            Shift::ROROrRRX => {
                let rs_immed_5 = rs_val & 0b1_1111;

                if rs_immed_8 == 0 {
                    Self {
                        val: rm_val,
                        shifter_carry_out: c_flag,
                    }
                } else if rs_immed_5 == 0 {
                    Self {
                        val: rm_val,
                        shifter_carry_out: BitState::from(rm_val >> 31),
                    }
                } else {
                    Self {
                        val: rm_val >> rs_immed_5,
                        shifter_carry_out: BitState::from(rm_val >> (rs_immed_5 - 1)),
                    }
                }
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

    // ----- Immediate (shift) tests -----
    // normal immediate test
    #[test]
    fn immediate() {
        let mut nds = NintendoDS::default();
        {
            let cpsr = nds.arm7tdmi.registers.get_mut_cpsr();
            cpsr.set_condition_bit(ConditionBit::C, BitState::Set);
        }

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_00000_000_1111,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_immediate(data);
        let expected_value = ShifterOperand {
            // remember: If (rm == PC) => Pc + 8
            val: 0x8,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
            );
    }

    // LSL => if shift_imm == 0
    #[test]
    fn immediate_shift_lsl1() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 1);
            {
                let cpsr = nds.arm7tdmi.registers.get_mut_cpsr();
                cpsr.set_condition_bit(ConditionBit::C, BitState::Set);
            }
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0000_000_0001,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            val: 0b1,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(
            value, expected_value,
            "{:#?} {:#?}",
            &value, &expected_value
            );
    }

    // LSL => else
    #[test]
    fn immediate_shift_lsl2() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rm
            nds.arm7tdmi
                .registers
                .set_reg(RegisterName::R0, 1);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_000_0000,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            val: 0b10,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
            );
    }

    // LSR => if shift_imm == 0
    #[test]
    fn immediate_shift_lsr1() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rm
            nds.arm7tdmi
                .registers
                .set_reg(RegisterName::R0, 1 << 31);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0000_010_0000,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            val: 0,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
            );
    }

    // LSR => else
    #[test]
    fn immediate_shift_lsr2() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rm
            nds.arm7tdmi
                .registers
                .set_reg(RegisterName::R2, 1 << 31);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_010_0010,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            val: 0b0100_0000_0000_0000__0000_0000_0000_0000,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(
            value, expected_value,
            "{:#?} {:#?}",
            &value, &expected_value
            );
    }

    // ASR => if shift_imm == 0 => if (rm >> 31) & 0b1 == 0
    #[test]
    fn immediate_shift_asr1() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rm 
            nds.arm7tdmi
                .registers
                .set_reg(RegisterName::R1, !(1 << 31));
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_00000_100_0001,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            val: 0,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
            );
    }

    // ASR => if shift_imm == 0 => else
    #[test]
    fn immediate_shift_asr2() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rm
            nds.arm7tdmi
                .registers
                .set_reg(RegisterName::R5, 1 << 31);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_00000_100_0101,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            val: 0xFFFF_FFFF,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(
            value, expected_value,
            "{:#?} {:#?}",
            &value, &expected_value
            );
    }

    // ASR => else
    #[test]
    fn immediate_shift_asr3() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R6, 0b1000_0000_0000_0000__0000_0000_0000_0001);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_00001_100_0110,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            val: (0b11) << 30,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(
            value, expected_value,
            "{:#?} {:#?}",
            &value, &expected_value
            );
    }

    // RRX
    #[test]
    fn immediate_shift_rrx() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rm
            nds.arm7tdmi
                .registers
                .set_reg(RegisterName::R7, 0b10);
            {
                let cpsr = nds.arm7tdmi.registers.get_mut_cpsr();
                cpsr.set_condition_bit(ConditionBit::C, BitState::Set);
            }
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0000_110_0111,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            val: 0b1000_0000_0000_0000__0000_0000_0000_0001,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
            );
    }

    // ROR
    #[test]
    fn immediate_shift_ror() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rm
            nds.arm7tdmi
                .registers
                .set_reg(RegisterName::R9, 0b11);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_110_1001,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_immediate_shift(data);
        let expected_value = ShifterOperand {
            val: 1,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
            );
    }

    // ----- Immediate (shift) tests -----
    /// LSL => if rs_immed_8 == 0
    #[test]
    fn register_shift_lsl1() {
        let nds = {
            let mut nds = NintendoDS::default();
            nds.arm7tdmi.registers.set_reg(RegisterName::R3, 42);
            {
                let cpsr = nds.arm7tdmi.registers.get_mut_cpsr();
                cpsr.set_condition_bit(ConditionBit::C, BitState::Set);
            }
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_00_1_0011,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };
        let value = ShifterOperand::get_register_shift(data);
        let expected_value = ShifterOperand {
            val: 42,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(
            value, expected_value,
            "{:#?} {:#?}",
            &value, &expected_value
            );
    }

    // LSL => else if rs_immed_8 < 32
    #[test]
    fn register_shift_lsl2() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 31);
            // rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R2, 1);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_00_1_0010,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);

        let expected_value = ShifterOperand {
            val: 1 << 31,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
            );
    }

    // LSL => else if rs_immed_8 == 32
    #[test]
    fn register_shift_lsl3() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 32);
            // rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R2, 1);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_00_1_0010,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);

        let expected_value = ShifterOperand {
            val: 0,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
            );
    }

    // LSL => else
    #[test]
    fn register_shift_lsl4() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 33);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_00_1_0000,
                ..Instruction::default()
            };

            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);
        let expected_value = ShifterOperand {
            val: 0,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
            );
    }

    // LSR => if rs_immed_8 == 0
    #[test]
    fn register_shift_lsr1() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi
                .registers
                .set_reg(RegisterName::R1, 0b1111_1111_1111_1111__1111_1111_0000_0000);
            // rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R2, 100);
            {
                let cpsr = nds.arm7tdmi.registers.get_mut_cpsr();
                cpsr.set_condition_bit(ConditionBit::C, BitState::Set);
            };
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_01_0_0010,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);
        let expected_value = ShifterOperand {
            val: 100,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
            );
    }

    // LSR => else if rs_immed_8 < 32
    #[test]
    fn register_shift_lsr2() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 1);
            // rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R2, 1 << 31);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_01_1_0010,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);
        let expected_value = ShifterOperand {
            val: 1 << 30,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
            );
    }

    // LSR => if rs_immed_8 == 32
    #[test]
    fn register_shift_lsr3() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 32);
            // rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R2, 1 << 31);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_01_1_0010,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);
        let expected_value = ShifterOperand {
            val: 0,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
            );
    }

    // LSR => else
    #[test]
    fn register_shift_lsr4() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 33);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_01_1_0000,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);
        let expected_value = ShifterOperand {
            val: 0,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
            );
    }

    // ASR => if rs_immed_8 == 0
    #[test]
    fn register_shift_asr1() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 0);
            // rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R2, u32::MAX);
            // set c_flag
            {
                let cpsr = nds.arm7tdmi.registers.get_mut_cpsr();
                cpsr.set_condition_bit(ConditionBit::C, BitState::Set);
            }
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0101_0010,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);
        let expected_value = ShifterOperand {
            val: u32::MAX,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
            );
    }

    // ASR => else if rs_immed_8 < 32
    #[test]
    fn register_shift_asr2() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 2);
            // rm
            nds.arm7tdmi
                .registers
                .set_reg(RegisterName::R2, 0b1000_0000_0000_0000__0000_0000_0000_0010);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_10_1_0010,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);
        let expected_value = ShifterOperand {
            val: 0b0010_0000_0000_0000__0000_0000_0000_0000,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
            );
    }

    // ASR => else => BitState::Unset
    #[test]
    fn register_shift_asr3() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 32);
            // rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R2, 0);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_10_1_0010,
                .. Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);
        let expected_value = ShifterOperand {
            val: 0,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(expected_value, value, "{:#?} {:#?}", &expected_value, &value);
    }

    // ASR => else => match BitState::from(rm_al >> 31) => BitState::Set
    #[test]
    fn register_shift_asr4() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 33);
            // rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R2, 1 << 31);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_10_1_0010,
                .. Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);
        let expected_value = ShifterOperand {
            val: 0xFFFF_FFFF,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(expected_value, value, "{:#?} {:#?}", &expected_value, &value);
    }

    // ROR => if rs_immed_8 == 0
    #[test]
    fn register_shift_ror1() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 0);
            // rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R2, u32::MAX);
            {
                let cpsr = nds.arm7tdmi.registers.get_mut_cpsr();
                cpsr.set_condition_bit(ConditionBit::C, BitState::Set);
            }
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_11_1_0010,
                .. Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);
        let expected_value = ShifterOperand {
            val: u32::MAX,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(expected_value, value, "{:#?} {:#?}", &expected_value, &value);
    }

    // ROR => else if rs_immed_5 == 0
    #[test]
    fn register_shift_ror2() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 0b0000_0000_0000_0000__0000_0000_1110_0000);
            // rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R2, u32::MAX);
            nds
        };
        
        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_11_1_0010,
                .. Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);
        let expected_value = ShifterOperand {
            val: u32::MAX,
            shifter_carry_out: BitState::Set,
        };

        assert_eq!(expected_value, value, "{:#?} {:#?}", &expected_value, &value);
    }

    // ROR => else
    #[test]
    fn register_shift_ror3() {
        let nds = {
            let mut nds = NintendoDS::default();
            // rs
            nds.arm7tdmi.registers.set_reg(RegisterName::R1, 1);
            // rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R2, 1 << 31);
            nds
        };

        let data = {
            let instruction = Instruction {
                val: 0b0000_000_0000_0_0000_0000_0001_0_11_1_0010,
                .. Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = ShifterOperand::get_register_shift(data);
        let expected_value = ShifterOperand {
            val: 1 << 30,
            shifter_carry_out: BitState::Unset,
        };

        assert_eq!(expected_value, value, "{:#?} {:#?}", &expected_value, &value);
    }
}
