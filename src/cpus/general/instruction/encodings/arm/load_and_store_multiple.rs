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
pub struct LoadAndStoreMultiple {
    p_flag: BitState,
    u_flag: BitState,
    s_flag: BitState,
    w_flag: BitState,
    l_flag: BitState,
    rn: u8,
    register_list: RegisterList,
}

impl<'a> From<DecodeData<'a>> for LoadAndStoreMultiple {
    fn from(data: DecodeData<'a>) -> Self {
        let p_flag = BitState::from(data.instruction.val >> 24);
        let u_flag = BitState::from(data.instruction.val >> 23);
        let s_flag = BitState::from(data.instruction.val >> 22);
        let w_flag = BitState::from(data.instruction.val >> 21);
        let l_flag = BitState::from(data.instruction.val >> 20);
        let rn = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let register_list = RegisterList::from(data.instruction.val);
        Self {
            p_flag,
            u_flag,
            s_flag,
            w_flag,
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
        LoadAndStoreMultiple,
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
            val: 0b0000_100_1_0_1_0_1_1111_1110_1100_1000_0000,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = LoadAndStoreMultiple::from(data);

        let expected_value = LoadAndStoreMultiple {
            p_flag: BitState::Set,
            u_flag: BitState::Unset,
            s_flag: BitState::Set,
            w_flag: BitState::Unset,
            l_flag: BitState::Set,
            rn: 0b1111,
            register_list: RegisterList::from(0b1110_1100_1000_0000),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
