use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreToFromStack {
    l_flag: BitState,
    rd: u8,
    sp_relative_offset: u8,
}

impl<'a> From<DecodeData<'a>> for LoadStoreToFromStack {
    fn from(data: DecodeData<'a>) -> Self {
        let l_flag = BitState::from(data.instruction.val >> 11);
        let rd = u8::try_from((data.instruction.val >> 8) & 0b111).unwrap();
        let sp_relative_offset = u8::try_from(data.instruction.val & 0b1111_1111).unwrap();
        Self {
            l_flag,
            rd,
            sp_relative_offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DecodeData,
        LoadStoreToFromStack,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b1001_1_101_1100_0011,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = LoadStoreToFromStack::from(data);

        let expected_value = LoadStoreToFromStack {
            l_flag: BitState::Set,
            rd: 0b0101,
            sp_relative_offset: 0b1100_0011,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
