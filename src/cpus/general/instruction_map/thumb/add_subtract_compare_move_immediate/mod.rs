use crate::cpus::general::instruction::Instruction;

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSubtractCompareMoveImmediate {
    opcode: u8,
    rd_rn: u8,
    immediate: u8,
}

impl From<&Instruction> for AddSubtractCompareMoveImmediate {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 11) & 0b11).unwrap();
        let rd_rn = u8::try_from((instruction_val >> 8) & 0b111).unwrap();
        let immediate = u8::try_from(instruction_val & 0b1111_1111).unwrap();
        Self {opcode, rd_rn, immediate}
    }
}

#[cfg(test)]
mod tests {
    use super::{AddSubtractCompareMoveImmediate, Instruction};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b001_11_110_1010_0101);
        let value = AddSubtractCompareMoveImmediate::from(&instruction);

        let expected_value = AddSubtractCompareMoveImmediate {
            opcode: 0b11,
            rd_rn: 0b110,
            immediate: 0b1010_0101,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
