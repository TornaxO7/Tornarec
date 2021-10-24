use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreHalfwordImmediateOffset {
    l_flag: BitState,
    offset: u8,
    rn: u8,
    rd: u8,
}

impl<'a> From<DecodeData<'a>> for LoadStoreHalfwordImmediateOffset {
    fn from(data: DecodeData<'a>) -> Self {
        let l_flag = BitState::from(data.instruction.val >> 11);
        let offset = u8::try_from((data.instruction.val >> 6) & 0b1_1111).unwrap();
        let rn = u8::try_from((data.instruction.val >> 3) & 0b111).unwrap();
        let rd = u8::try_from(data.instruction.val & 0b111).unwrap();
        Self {
            l_flag,
            offset,
            rn,
            rd,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DecodeData,
        LoadStoreHalfwordImmediateOffset,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b1000_1_10101_110_100,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = LoadStoreHalfwordImmediateOffset::from(data);

        let expected_value = LoadStoreHalfwordImmediateOffset {
            l_flag: BitState::Set,
            offset: 0b10101,
            rn: 0b0110,
            rd: 0b0100,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
