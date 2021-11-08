use crate::cpus::general::{instruction::decode::DecodeData, register::RegisterName};

use super::error::MiscellaneousError;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountLeadingZeros {
    pub rd_reg: RegisterName,
    pub rm_reg: RegisterName,
}

impl<'a> From<DecodeData<'a>> for CountLeadingZeros {
    fn from(data: DecodeData<'a>) -> Self {
        let sbo1 = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let rd_reg = RegisterName::from((data.instruction.val >> 12) & 0b1111);
        let sbo2 = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let rm_reg = RegisterName::from(data.instruction.val & 0b1111);

        if sbo1 != 0b1111 || sbo2 != 0b1111 {
            unreachable!("{}", MiscellaneousError::SBOConflict(data.instruction.val));
        }

        Self { rd_reg, rm_reg }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    use super::{
        CountLeadingZeros,
        DecodeData,
        RegisterName,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let data = {
            let instruction = Instruction {
                val: 0b0000_00010_110_1111_1100_1111_0001_1010,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = CountLeadingZeros::from(data);
        let expected_value = CountLeadingZeros {
            rd_reg: RegisterName::R12,
            rm_reg: RegisterName::R10,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
        );
    }
}
