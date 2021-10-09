use crate::cpus::general::instruction::Instruction;

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShiftByImmediate {
    opcode: u8,
    immediate: u8,
    rm: u8,
    rd: u8,
}

impl From<&Instruction> for ShiftByImmediate {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 11) & 0b11).unwrap();
        let immediate = u8::try_from((instruction_val >> 6) & 0b11111).unwrap();
        let rm = u8::try_from((instruction_val >> 3) & 0b111).unwrap();
        let rd = u8::try_from(instruction_val & 0b111).unwrap();
        Self {opcode, immediate, rm, rd}
    }
}

#[cfg(test)]
mod tests {
    use super::{ShiftByImmediate, Instruction};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b000_11_10101_101_010);
        let value = ShiftByImmediate::from(&instruction);

        let expected_value = ShiftByImmediate {
            opcode: 0b11,
            immediate: 0b10101,
            rm: 0b101,
            rd: 0b010,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
