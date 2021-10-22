use crate::cpus::general::instruction::Instruction;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadFromLiteralPool {
    rd: u8,
    pc_relative_offset: u8,
}

impl From<&Instruction> for LoadFromLiteralPool {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let rd = u8::try_from((instruction_val >> 8) & 0b111).unwrap();
        let pc_relative_offset = u8::try_from(instruction_val & 0b1111_1111).unwrap();
        Self {
            rd,
            pc_relative_offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Instruction,
        LoadFromLiteralPool,
    };

    #[test]
    fn from() {
        let instruction = Instruction::from(0b01001_111_1010_0101);
        let value = LoadFromLiteralPool::from(&instruction);

        let expected_value = LoadFromLiteralPool {
            rd: 0b111,
            pc_relative_offset: 0b1010_0101,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
