use crate::cpus::general::instruction::decode::DecodeData;

use super::error::MiscellaneousError;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountLeadingZeros {
    pub rd: u8,
    pub rm: u8,
}

impl<'a> From<DecodeData<'a>> for CountLeadingZeros {
    fn from(data: DecodeData<'a>) -> Self {
        let sbo1 = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let sbo2 = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let rm = u8::try_from(data.instruction.val & 0b1111).unwrap();

        if sbo1 != 0b1111 || sbo2 != 0b1111 {
            unreachable!("{}", MiscellaneousError::SBOConflict(data.instruction.val));
        }

        Self { rd, rm }
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
            rd: 0b1100,
            rm: 0b1010,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
        );
    }
}
