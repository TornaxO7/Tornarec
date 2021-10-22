use crate::cpus::general::{
    instruction::{
        encodings::encoding_fields::RegisterList,
        decode::DecodeData,
    },
    register::NormalizedRegister,
    BitState,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreMultiple {
    l_flag: BitState,
    rn: NormalizedRegister,
    register_list: RegisterList,
}

impl<'a> From<DecodeData<'a>> for LoadStoreMultiple {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

        let l_flag = BitState::from(instruction_val >> 11);
        let rn = NormalizedRegister::from((instruction_val >> 8) & 0b111);
        let register_list = RegisterList::from(instruction_val & 0b1111_1111);
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
        NormalizedRegister,
        RegisterList,
    };

    use crate::{
        cpus::general::{
            register::RegisterName,
            Instruction,
        },
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b1100_1_101_1111_0011);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = LoadStoreMultiple::from(data);

        let expected_value = LoadStoreMultiple {
            l_flag: BitState::Set,
            rn: NormalizedRegister::from(RegisterName::R5),
            register_list: RegisterList::from(0b1111_0011),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
