use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalBranch {
    cond: u8,
    offset: u8,
}

impl<'a> From<DecodeData<'a>> for ConditionalBranch {
    fn from(data: DecodeData<'a>) -> Self {
        let cond = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let offset = u8::try_from(data.instruction.val & 0b1111_1111).unwrap();
        Self { cond, offset }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ConditionalBranch,
        DecodeData,
    };

    use crate::{
        NintendoDS,
        cpus::general::Instruction,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b1101_1001_1110_1100,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = ConditionalBranch::from(data);

        let expected_value = ConditionalBranch {
            cond: 0b1001,
            offset: 0b1110_1100,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
