use crate::cpus::general::{
    BitState,
    instruction::Instruction,
    instruction_encodings::encoding_fields::RegisterList,
};

use std::convert::{From, TryFrom};

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

impl From<&Instruction> for LoadAndStoreMultiple {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let p_flag = BitState::from(instruction_val >> 24);
        let u_flag = BitState::from(instruction_val >> 23);
        let s_flag = BitState::from(instruction_val >> 22);
        let w_flag = BitState::from(instruction_val >> 21);
        let l_flag = BitState::from(instruction_val >> 20);
        let rn = u8::try_from((instruction_val >> 16) & 0b1111).unwrap();
        let register_list = RegisterList::from(instruction_val);
        Self{p_flag, u_flag, s_flag, w_flag, l_flag, rn, register_list}
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadAndStoreMultiple, Instruction, BitState, RegisterList};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b0000_100_1_0_1_0_1_1111_1110_1100_1000_0000);
        let value = LoadAndStoreMultiple::from(&instruction);

        let expected_value = LoadAndStoreMultiple {
            p_flag: BitState::Set,
            u_flag: BitState::Unset,
            s_flag: BitState::Set,
            w_flag: BitState::Unset,
            l_flag: BitState::Set,
            rn: 0b1111,
            register_list: RegisterList::from(0b1110_1100_1000_0000),
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
