use crate::cpus::general::{
    instruction::{
        Instruction,
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

impl From<&Instruction> for PushPopRegisterList {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let l_flag = BitState::from(instruction_val >> 11);
        let r_flag = BitState::from(instruction_val >> 8);
        let register_list = RegisterList::from(instruction_val & 0b1111_1111);
        Self {l_flag, r_flag, register_list}
    }
}

#[cfg(test)]
mod tests {
    use super::{PushPopRegisterList, Instruction, BitState, RegisterList};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b1011_1_10_0_1111_0000);
        let value = PushPopRegisterList::from(&instruction);

        let expected_value = PushPopRegisterList {
            l_flag: BitState::Set,
            r_flag: BitState::Unset,
            register_list: RegisterList::from(0b1111_0000),
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
