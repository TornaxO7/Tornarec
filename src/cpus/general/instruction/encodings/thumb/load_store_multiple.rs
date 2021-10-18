use crate::cpus::general::{
    BitState,
    instruction::{
        Instruction,
        encodings::encoding_fields::RegisterList,
    },
    register::RegisterName,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreMultiple {
    l_flag: BitState,
    rn: RegisterName,
    register_list: RegisterList
}

impl From<&Instruction> for LoadStoreMultiple {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let l_flag = BitState::from(instruction_val >> 11);
        let rn = RegisterName::from((instruction_val >> 8) & 0b111);
        let register_list = RegisterList::from(instruction_val & 0b1111_1111);
        Self {l_flag, rn, register_list}
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadStoreMultiple, Instruction, BitState, RegisterList, RegisterName};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b1100_1_101_1111_0011);
        let value = LoadStoreMultiple::from(&instruction);
        
        let expected_value = LoadStoreMultiple {
            l_flag: BitState::Set,
            rn: RegisterName::R5,
            register_list: RegisterList::from(0b1111_0011),
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
