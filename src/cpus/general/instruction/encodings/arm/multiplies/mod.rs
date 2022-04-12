mod multiply_instruction;

pub use multiply_instruction::MultiplyInstruction;

use crate::cpus::general::{BitState, instruction::decode::DecodeData, register::RegisterName};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Multiplies {
    pub multiply_instruction: MultiplyInstruction,
    pub s_flag: BitState,
    pub rn_reg: RegisterName,
    pub rd_reg: RegisterName,
    pub rs_reg: RegisterName,
    pub rm_reg: RegisterName,
}

impl<'a> From<DecodeData<'a>> for Multiplies {
    fn from(data: DecodeData<'a>) -> Self {
        let multiply_instruction = MultiplyInstruction::from(&data.instruction);
        let s_flag = BitState::from(data.instruction.val >> 20);
        let rn_reg = RegisterName::from((data.instruction.val >> 16) & 0b1111);
        let rd_reg = RegisterName::from((data.instruction.val >> 12) & 0b1111);
        let rs_reg = RegisterName::from((data.instruction.val >> 8) & 0b1111);
        let rm_reg = RegisterName::from(data.instruction.val & 0b1111);

        Self {
            multiply_instruction,
            s_flag,
            rn_reg,
            rd_reg,
            rs_reg,
            rm_reg,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DecodeData,
        Multiplies,
        RegisterName,
        MultiplyInstruction,
    };

    use crate::{NintendoDS, cpus::general::{BitState, Instruction}};

    #[test]
    fn test_from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b0000_0000_1111_1100_0011_1110_1001_0110,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = Multiplies::from(data);

        let expected_value = Multiplies {
            multiply_instruction: MultiplyInstruction::SMLAL,
            s_flag: BitState::Set,
            rn_reg: RegisterName::R12,
            rd_reg: RegisterName::R3,
            rs_reg: RegisterName::R14,
            rm_reg: RegisterName::R6,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}