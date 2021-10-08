use crate::cpus::general::{instruction::Instruction, BitState};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadAndStoreImmediateOffset {
    p_flag: BitState,
    u_flag: BitState,
    b_flag: BitState,
    w_flag: BitState,
    l_flag: BitState,
    rn: u8,
    rd: u8,
    immediate: u16,
}

impl From<&Instruction> for LoadAndStoreImmediateOffset {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let p_flag = BitState::from(instruction_val >> 24);
        let u_flag = BitState::from(instruction_val >> 23);
        let b_flag = BitState::from(instruction_val >> 22);
        let w_flag = BitState::from(instruction_val >> 21);
        let l_flag = BitState::from(instruction_val >> 20);
        let rn = u8::try_from((instruction_val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((instruction_val >> 12) & 0b1111).unwrap();
        let immediate = u16::try_from(instruction_val & 0b1111_1111_1111).unwrap();
        Self { p_flag, u_flag, b_flag, w_flag, l_flag, rn, rd, immediate }
    }
}

#[cfg(test)]
mod tests {
    use super::{BitState, Instruction, LoadAndStoreImmediateOffset};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b0000_010_1_0_1_0_1_1100_0011_1110_1100_1000);
        let value = LoadAndStoreImmediateOffset::from(&instruction);

        let expected_value = LoadAndStoreImmediateOffset {
            p_flag: BitState::Set,
            u_flag: BitState::Unset,
            b_flag: BitState::Set,
            w_flag: BitState::Unset,
            l_flag: BitState::Set,
            rn: 0b1100,
            rd: 0b0011,
            immediate: 0b1110_1100_1000,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
