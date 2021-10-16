use crate::cpus::general::{
    instruction::Instruction,
    BitState,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdjustStackPointer {
    opc: BitState,
    immediate: u8,
}

impl From<&Instruction> for AdjustStackPointer {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opc = BitState::from(instruction_val >> 7);
        let immediate = u8::try_from(instruction_val & 0b111_1111).unwrap();
        Self {opc, immediate}
    }
}

#[cfg(test)]
mod tests {
    use super::{AdjustStackPointer, Instruction, BitState};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b1011_0000_1_100_1000);
        let value = AdjustStackPointer::from(&instruction);

        let expected_value = AdjustStackPointer {
            opc: BitState::Set,
            immediate: 0b100_1000,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
