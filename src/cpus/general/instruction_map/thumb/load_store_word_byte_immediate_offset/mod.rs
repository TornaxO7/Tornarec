use crate::cpus::general::{
    instruction::Instruction,
    BitState,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreWordByteImmediateOffset {
    b_flag: BitState,
    l_flag: BitState,
    offset: u8,
    rn: u8,
    rd: u8,
}

impl From<&Instruction> for LoadStoreWordByteImmediateOffset {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let b_flag = BitState::from(instruction_val >> 12);
        let l_flag = BitState::from(instruction_val >> 11);
        let offset = u8::try_from((instruction_val >> 6) & 0b1_1111).unwrap();
        let rn = u8::try_from((instruction_val >> 3) & 0b111).unwrap();
        let rd = u8::try_from(instruction_val & 0b111).unwrap();
        Self {b_flag, l_flag, offset, rn, rd}
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadStoreWordByteImmediateOffset, Instruction, BitState};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b011_1_0_11100_101_010);
        let value = LoadStoreWordByteImmediateOffset::from(&instruction);

        let expected_value = LoadStoreWordByteImmediateOffset {
            b_flag: BitState::Set,
            l_flag: BitState::Unset,
            offset: 0b11100,
            rn: 0b101,
            rd: 0b010,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
