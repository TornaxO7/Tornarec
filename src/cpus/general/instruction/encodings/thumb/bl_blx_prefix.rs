use crate::cpus::general::instruction::Instruction;

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlOrBlxPrefix {
    offset: u16,
}

impl From<&Instruction> for BlOrBlxPrefix {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let offset = u16::try_from(instruction_val & 0b111_1111_1111).unwrap();
        Self {offset}
    }
}

#[cfg(test)]
mod tests {
    use super::{BlOrBlxPrefix, Instruction};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b11110_100_1010_0101);
        let value = BlOrBlxPrefix::from(&instruction);

        let expected_value = BlOrBlxPrefix {
            offset: 0b100_1010_0101,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
