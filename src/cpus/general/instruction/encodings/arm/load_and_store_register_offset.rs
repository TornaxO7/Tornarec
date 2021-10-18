use crate::cpus::general::{
    BitState,
    instruction::Instruction,
    register::RegisterName,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadAndStoreRegisterOffset {
    p_flag: BitState,
    u_flag: BitState,
    b_flag: BitState,
    w_flag: BitState,
    l_flag: BitState,
    rn: RegisterName,
    rd: RegisterName,
    shift_amount: u8,
    shift: u8,
    rm: RegisterName,
}

impl From<&Instruction> for LoadAndStoreRegisterOffset {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let p_flag = BitState::from(instruction_val >> 24);
        let u_flag = BitState::from(instruction_val >> 23);
        let b_flag = BitState::from(instruction_val >> 22);
        let w_flag = BitState::from(instruction_val >> 21);
        let l_flag = BitState::from(instruction_val >> 20);
        let rn = RegisterName::from((instruction_val >> 16) & 0b1111);
        let rd = RegisterName::from((instruction_val >> 12) & 0b1111);
        let shift_amount = u8::try_from((instruction_val >> 7) & 0b1_1111).unwrap();
        let shift = u8::try_from((instruction_val >> 5) & 0b11).unwrap();
        let rm = RegisterName::from(instruction_val & 0b1111);
        Self{p_flag, u_flag, b_flag, w_flag, l_flag, rn, rd, shift_amount, shift, rm}
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadAndStoreRegisterOffset, BitState, Instruction, RegisterName};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b0000_011_1_0_1_0_1_1100_0011_11100_01_0_1010);
        let value = LoadAndStoreRegisterOffset::from(&instruction);

        let expected_value = LoadAndStoreRegisterOffset {
            p_flag: BitState::Set,
            u_flag: BitState::Unset,
            b_flag: BitState::Set,
            w_flag: BitState::Unset,
            l_flag: BitState::Set,
            rn: RegisterName::R12,
            rd: RegisterName::R3,
            shift_amount: 0b11100,
            shift: 0b01,
            rm: RegisterName::R10,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
