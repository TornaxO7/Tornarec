use crate::cpus::general::instruction::Instruction;

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlBlxPrefix {
    offset: u16,
}

impl From<&Instruction> for BlBlxPrefix {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let offset = u16::try_from(instruction_val & 0b111_1111_1111).unwrap();
        Self {offset}
    }
}

#[cfg(test)]
mod tests {
    use super::{BlBlxPrefix, Instruction};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b11110_100_1010_0101);
        let value = BlBlxPrefix::from(&instruction);

        let expected_value = BlBlxPrefix {
            offset: 0b100_1010_0101,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
