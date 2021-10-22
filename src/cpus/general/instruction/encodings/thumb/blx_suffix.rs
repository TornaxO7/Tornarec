use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlxSuffix {
    offset: u16,
}

impl<'a> From<DecodeData<'a>> for BlxSuffix {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

        let offset = u16::try_from((instruction_val >> 1) & 0b11_1111_1111).unwrap();
        Self { offset }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BlxSuffix,
        Instruction,
    };

    #[test]
    fn from() {
        let instruction = Instruction::from(0b11101_10_1010_1010_0);
        let value = BlxSuffix::from(&instruction);

        let expected_value = BlxSuffix {
            offset: 0b10_1010_1010,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
