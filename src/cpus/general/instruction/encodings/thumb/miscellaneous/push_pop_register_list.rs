use crate::cpus::general::{
    instruction::{
        decode::DecodeData,
        encodings::encoding_fields::RegisterList,
    },
    BitState,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushPopRegisterList {
    l_flag: BitState,
    r_flag: BitState,
    register_list: RegisterList,
}

impl<'a> From<DecodeData<'a>> for PushPopRegisterList {
    fn from(data: DecodeData<'a>) -> Self {
        let l_flag = BitState::from(data.instruction.val >> 11);
        let r_flag = BitState::from(data.instruction.val >> 8);
        let register_list = RegisterList::from(data.instruction.val & 0b1111_1111);
        Self {
            l_flag,
            r_flag,
            register_list,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DecodeData,
        PushPopRegisterList,
        RegisterList,
    };

    use crate::{
        NintendoDS,
        cpus::general::Instruction,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b1011_1_10_0_1111_0000,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = PushPopRegisterList::from(data);

        let expected_value = PushPopRegisterList {
            l_flag: BitState::Set,
            r_flag: BitState::Unset,
            register_list: RegisterList::from(0b1111_0000),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
