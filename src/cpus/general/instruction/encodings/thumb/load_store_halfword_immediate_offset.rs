use crate::cpus::general::{
    instruction::Instruction,
    BitState,
    register::RegisterName,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreHalfwordImmediateOffset {
    l_flag: BitState,
    offset: u8,
    rn: RegisterName,
    rd: RegisterName,
}

impl From<&Instruction> for LoadStoreHalfwordImmediateOffset {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let l_flag = BitState::from(instruction_val >> 11);
        let offset = u8::try_from((instruction_val >> 6) & 0b1_1111).unwrap();
        let rn = RegisterName::from((instruction_val >> 3) & 0b111);
        let rd = RegisterName::from(instruction_val & 0b111);
        Self {l_flag, offset, rn, rd}
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadStoreHalfwordImmediateOffset, BitState, Instruction, RegisterName};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b1000_1_10101_110_100);
        let value = LoadStoreHalfwordImmediateOffset::from(&instruction);
        
        let expected_value = LoadStoreHalfwordImmediateOffset {
            l_flag: BitState::Set,
            offset: 0b10101,
            rn: RegisterName::R6,
            rd: RegisterName::R4,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
