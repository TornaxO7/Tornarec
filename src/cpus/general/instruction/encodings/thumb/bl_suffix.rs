use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlSuffix {
    offset: u16,
}

impl<'a> From<DecodeData<'a>> for BlSuffix {
    fn from(data: DecodeData<'a>) -> Self {
        let offset = u16::try_from(data.instruction.val & 0b111_1111_1111).unwrap();
        Self { offset }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BlSuffix,
        DecodeData,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b11111_100_1000_0001,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = BlSuffix::from(data);

        let expected_value = BlSuffix {
            offset: 0b100_1000_0001,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
