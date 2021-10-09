use crate::cpus::general::{
    instruction::Instruction,
    BitState,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddToSpOrPc {
    sp: BitState,
    rd: u8,
    immediate: u8,
}

impl From<&Instruction> for AddToSpOrPc {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let sp = BitState::from(instruction_val >> 11);
        let rd = u8::try_from((instruction_val >> 8) & 0b111).unwrap();
        let immediate = u8::try_from(instruction_val & 0b1111_1111).unwrap();
        Self {sp, rd, immediate}
    }
}

#[cfg(test)]
mod tests {
    use super::{AddToSpOrPc, Instruction, BitState};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b1011_1_101_1100_1000);
        let value = AddToSpOrPc::from(&instruction);
        
        let expected_value = AddToSpOrPc {
            sp: BitState::Set,
            rd: 0b101,
            immediate: 0b1100_1000,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
