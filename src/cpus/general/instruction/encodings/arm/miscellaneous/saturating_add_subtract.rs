use super::error::MiscellaneousError;

use std::convert::{
    From,
    TryFrom,
};

use crate::cpus::general::instruction::decode::DecodeData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaturatingAddSubtract {
    op: u8,
    rn: u8,
    rd: u8,
    rm: u8,
}

impl<'a> From<DecodeData<'a>> for SaturatingAddSubtract {
    fn from(data: DecodeData<'a>) -> Self {
        let op = u8::try_from((data.instruction.val >> 21) & 0b11).unwrap();
        let rn = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let sbz = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let rm = u8::try_from(data.instruction.val & 0b1111).unwrap();

        if sbz != 0 {
            unreachable!("{}", MiscellaneousError::SBZConflict(data.instruction.val));
        }

        Self { op, rn, rd, rm }
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
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let data = {
            let instruction = Instruction {
                val: 0b0000_00010_11_0_1101_1100_0000_0101_1000,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = SaturatingAddSubtract::from(data);
        let expected_value = SaturatingAddSubtract {
            op: 0b11,
            rn: 0b1101,
            rd: 0b1100,
            rm: 0b1000,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
        );
    }
}
