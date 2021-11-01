use crate::cpus::general::{
    instruction::decode::DecodeData,
    register::RegisterName,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveToStatusRegister {
    pub r_flag: BitState,
    pub field_mask: u8,
    // The value of bit 0 to bit 7 (including)
    pub operand: u32,
}

impl<'a> From<DecodeData<'a>> for MoveToStatusRegister {
    fn from(data: DecodeData<'a>) -> Self {
        let is_immediate_operand = (data.instruction.val >> 25) & 1 == 1;

        let r_flag = BitState::from(data.instruction.val >> 22);
        let field_mask = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();

        let operand = {
            if is_immediate_operand {
                let rotate_imm = (data.instruction.val >> 8) & 0b1111;
                let immediate = data.instruction.val & 0b1111_1111;

                immediate.rotate_right(rotate_imm * 2)
            } else {
                let rm = RegisterName::from(data.instruction.val & 0b1111);
                data.registers.get_reg(rm)
            }
        };

        Self {
            r_flag,
            field_mask,
            operand,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DecodeData,
        MoveToStatusRegister,
    };

    use crate::{NintendoDS, cpus::general::{Instruction, register::RegisterName}};

    #[test]
    fn immediate() {
        let nds = NintendoDS::default();
        let data = {
            let instruction = Instruction {
                val: 0b0000_00110_1_10_1111_1111_0001_0000_0001,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = MoveToStatusRegister::from(data);
        let expected_value = MoveToStatusRegister {
            r_flag: BitState::Set,
            field_mask: 0b1111,
            operand: 0b0100_0000_0000_0000__0000_0000_0000_0000,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
        );
    }

    #[test]
    fn register() {
        let nds = {
            let mut nds = NintendoDS::default();
            // set rm
            nds.arm7tdmi.registers.set_reg(RegisterName::R13, 42);
            nds
        };
        let data = {
            let instruction = Instruction {
                val: 0b0000_00010_1_10_1111_1111_0000_0000_1101,
                .. Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = MoveToStatusRegister::from(data);
        let expected_value = MoveToStatusRegister {
            r_flag: BitState::Set,
            field_mask: 0b1111,
            operand: 42,
        };

        assert_eq!(expected_value, value, "{:#?} {:#?}", &expected_value, &value);
    }
}
