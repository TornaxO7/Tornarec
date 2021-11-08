use super::error::MiscellaneousError;

use std::convert::{
    From,
    TryFrom,
};

use crate::cpus::general::{instruction::{decode::DecodeData, encodings::encoding_fields::SaturatingOpcode}, register::RegisterName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaturatingAddSubtract {
    pub opcode: SaturatingOpcode,
    pub rn_reg: RegisterName,
    pub rd_reg: RegisterName,
    pub rm_reg: RegisterName,
}

impl<'a> From<DecodeData<'a>> for SaturatingAddSubtract {
    fn from(data: DecodeData<'a>) -> Self {
        let op = SaturatingOpcode::from(data.instruction.val >> 21);
        let rn_reg = RegisterName::from((data.instruction.val >> 16) & 0b1111);
        let rd_reg = RegisterName::from((data.instruction.val >> 12) & 0b1111);
        let sbz = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let rm_reg = RegisterName::from(data.instruction.val & 0b1111);

        if sbz != 0 {
            unreachable!("{}", MiscellaneousError::SBZConflict(data.instruction.val));
        }

        Self { opcode: op, rn_reg, rd_reg, rm_reg }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    use super::{
        DecodeData,
        SaturatingAddSubtract,
        SaturatingOpcode,
        RegisterName,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let data = {
            let instruction = Instruction {
                val: 0b0000_00010_00_0_1101_1100_0000_0101_1000,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = SaturatingAddSubtract::from(data);
        let expected_value = SaturatingAddSubtract {
            opcode: SaturatingOpcode::QADD,
            rn_reg: RegisterName::R13,
            rd_reg: RegisterName::R12,
            rm_reg: RegisterName::R8,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
        );
    }
}
