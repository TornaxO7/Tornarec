mod signed_multiplies_opcode;

pub use signed_multiplies_opcode::SignedMultipliesOpcode;

use crate::cpus::general::{
    instruction::decode::DecodeData,
    register::RegisterName,
    BitState,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedMultipliesType2 {
    pub opcode: SignedMultipliesOpcode,
    pub rd_reg: RegisterName,
    pub rn_reg: RegisterName,
    pub rs_reg: RegisterName,
    pub y: BitState,
    pub x: BitState,
    pub rm_reg: RegisterName,
}

impl<'a> From<DecodeData<'a>> for SignedMultipliesType2 {
    fn from(data: DecodeData<'a>) -> Self {
        let opcode = SignedMultipliesOpcode::from(&data.instruction);
        let rd_reg = RegisterName::from((data.instruction.val >> 16) & 0b1111);
        let rn_reg = RegisterName::from((data.instruction.val >> 12) & 0b1111);
        let rs_reg = RegisterName::from((data.instruction.val >> 8) & 0b1111);
        let y = BitState::from(data.instruction.val >> 6);
        let x = BitState::from(data.instruction.val >> 5);
        let rm_reg = RegisterName::from(data.instruction.val & 0b1111);

        Self {
            opcode,
            rd_reg,
            rn_reg,
            rs_reg,
            y,
            x,
            rm_reg,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    use super::{
        BitState,
        DecodeData,
        RegisterName,
        SignedMultipliesOpcode,
        SignedMultipliesType2,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let data = {
            let instruction = Instruction {
                val: 0b0000_00010_110_1111_1110_1100_1110_1000,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = SignedMultipliesType2::from(data);
        let expected_value = SignedMultipliesType2 {
            opcode: SignedMultipliesOpcode::SMUL,
            rd_reg: RegisterName::R15,
            rn_reg: RegisterName::R14,
            rs_reg: RegisterName::R12,
            y: BitState::Set,
            x: BitState::Set,
            rm_reg: RegisterName::R8,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
        );
    }
}
