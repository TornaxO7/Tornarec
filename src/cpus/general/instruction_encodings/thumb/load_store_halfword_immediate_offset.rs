use crate::cpus::general::{
    instruction::Instruction,
    BitState,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreHalfwordImmediateOffset {
    l_flag: BitState,
    offset: u8,
    rn: u8,
    rd: u8,
}

impl From<&Instruction> for LoadStoreHalfwordImmediateOffset {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let l_flag = BitState::from(instruction_val >> 11);
        let offset = u8::try_from((instruction_val >> 6) & 0b1_1111).unwrap();
        let rn = u8::try_from((instruction_val >> 3) & 0b111).unwrap();
        let rd = u8::try_from(instruction_val & 0b111).unwrap();
        Self {l_flag, offset, rn, rd}
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadStoreHalfwordImmediateOffset, BitState, Instruction};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b1000_1_10101_110_100);
        let value = LoadStoreHalfwordImmediateOffset::from(&instruction);
        
        let expected_value = LoadStoreHalfwordImmediateOffset {
            l_flag: BitState::Set,
            offset: 0b10101,
            rn: 0b110,
            rd: 0b100,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
