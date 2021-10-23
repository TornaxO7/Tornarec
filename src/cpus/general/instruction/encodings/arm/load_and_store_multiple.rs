use crate::cpus::general::{
    BitState,
    instruction::{
        decode::DecodeData,
        encodings::encoding_fields::RegisterList,
    },
    register::NormalizedRegister,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadAndStoreMultiple {
    p_flag: BitState,
    u_flag: BitState,
    s_flag: BitState,
    w_flag: BitState,
    l_flag: BitState,
    rn: NormalizedRegister,
    register_list: RegisterList,
}

impl From<DecodeData> for LoadAndStoreMultiple {
    fn from(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();

        let p_flag = BitState::from(instruction_val >> 24);
        let u_flag = BitState::from(instruction_val >> 23);
        let s_flag = BitState::from(instruction_val >> 22);
        let w_flag = BitState::from(instruction_val >> 21);
        let l_flag = BitState::from(instruction_val >> 20);
        let rn = NormalizedRegister::from((instruction_val >> 16) & 0b1111);
        let register_list = RegisterList::from(instruction_val);
        Self{p_flag, u_flag, s_flag, w_flag, l_flag, rn, register_list}
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LoadAndStoreMultiple,
        BitState,
        RegisterList,
        DecodeData,
        NormalizedRegister
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
        let instruction = Instruction::from(0b0000_100_1_0_1_0_1_1111_1110_1100_1000_0000);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = LoadAndStoreMultiple::from(data);

        let expected_value = LoadAndStoreMultiple {
            p_flag: BitState::Set,
            u_flag: BitState::Unset,
            s_flag: BitState::Set,
            w_flag: BitState::Unset,
            l_flag: BitState::Set,
            rn: NormalizedRegister::from(RegisterName::R15),
            register_list: RegisterList::from(0b1110_1100_1000_0000),
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
