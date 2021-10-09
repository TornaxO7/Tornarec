use crate::cpus::general::instruction::Instruction;

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalBranch {
    cond: u8,
    offset: u8,
}

impl From<&Instruction> for ConditionalBranch {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let cond = u8::try_from((instruction_val >> 8) & 0b1111).unwrap();
        let offset = u8::try_from(instruction_val & 0b1111_1111).unwrap();
        Self {cond, offset}
    }
}

#[cfg(test)]
mod tests {
    use super::{ConditionalBranch, Instruction};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b1101_1001_1110_1100);
        let value = ConditionalBranch::from(&instruction);

        let expected_value = ConditionalBranch {
            cond: 0b1001,
            offset: 0b1110_1100,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
