use crate::cpus::general::{
    instruction::{
        decode::DecodeData,
        encodings::encoding_fields::RegisterList,
    },
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreMultiple {
    l_flag: BitState,
    rn: u8,
    register_list: RegisterList,
}

impl<'a> From<DecodeData<'a>> for LoadStoreMultiple {
    fn from(data: DecodeData<'a>) -> Self {
        let l_flag = BitState::from(data.instruction.val >> 11);
        let rn = u8::try_from((data.instruction.val >> 8) & 0b111).unwrap();
        let register_list = RegisterList::from(data.instruction.val & 0b1111_1111);
        Self {
            l_flag,
            rn,
            register_list,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DecodeData,
        LoadStoreMultiple,
        RegisterList,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b1100_1_101_1111_0011,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = LoadStoreMultiple::from(data);

        let expected_value = LoadStoreMultiple {
            l_flag: BitState::Set,
            rn: 0b0101,
            register_list: RegisterList::from(0b1111_0011),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
