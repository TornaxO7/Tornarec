use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlOrBlxPrefix {
    offset: u16,
}

impl<'a> From<DecodeData<'a>> for BlOrBlxPrefix {
    fn from(data: DecodeData<'a>) -> Self {
        let offset = u16::try_from(data.instruction.val & 0b111_1111_1111).unwrap();
        Self { offset }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BlOrBlxPrefix,
        DecodeData,
    };

    use crate::{
        NintendoDS,
        cpus::general::Instruction,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b11110_100_1010_0101);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = BlOrBlxPrefix::from(data);

        let expected_value = BlOrBlxPrefix {
            offset: 0b100_1010_0101,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
