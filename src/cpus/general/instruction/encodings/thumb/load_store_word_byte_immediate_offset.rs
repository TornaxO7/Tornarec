use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreWordByteImmediateOffset {
    b_flag: BitState,
    l_flag: BitState,
    offset: u8,
    rn: u8,
    rd: u8,
}

impl<'a> From<DecodeData<'a>> for LoadStoreWordByteImmediateOffset {
    fn from(data: DecodeData<'a>) -> Self {
        let b_flag = BitState::from(data.instruction.val >> 12);
        let l_flag = BitState::from(data.instruction.val >> 11);
        let offset = u8::try_from((data.instruction.val >> 6) & 0b1_1111).unwrap();
        let rn = u8::try_from((data.instruction.val >> 3) & 0b111).unwrap();
        let rd = u8::try_from(data.instruction.val & 0b111).unwrap();
        Self {
            b_flag,
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
        LoadStoreWordByteImmediateOffset,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b011_1_0_11100_101_010,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = LoadStoreWordByteImmediateOffset::from(data);

        let expected_value = LoadStoreWordByteImmediateOffset {
            b_flag: BitState::Set,
            l_flag: BitState::Unset,
            offset: 0b11100,
            rn: 0b0101,
            rd: 0b0010,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
