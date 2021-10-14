use crate::cpus::general::{
    instruction::Instruction,
    BitState,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialDataProcessing {
    opcode: u8,
    h1: BitState,
    h2: BitState,
    rm: u8,
    rd_rn: u8,
}

impl From<&Instruction> for SpecialDataProcessing {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 8) & 0b11).unwrap();
        let h1 = BitState::from(instruction_val >> 7);
        let h2 = BitState::from(instruction_val >> 6);
        let rm = u8::try_from((instruction_val >> 3) & 0b111).unwrap();
        let rd_rn = u8::try_from(instruction_val & 0b111).unwrap();
        Self {opcode, h1, h2, rm, rd_rn}
    }
}

#[cfg(test)]
mod tests {
    use super::{SpecialDataProcessing, Instruction, BitState};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b010001_11_1_0_101_010);
        let value = SpecialDataProcessing::from(&instruction);
        
        let expected_value = SpecialDataProcessing {
            opcode: 0b11,
            h1: BitState::Set,
            h2: BitState::Unset,
            rm: 0b101,
            rd_rn: 0b010,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
