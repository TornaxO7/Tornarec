use crate::cpus::general::instruction::Instruction;

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoftwareInterrupt {
    immediate: u8,
}

impl From<&Instruction> for SoftwareInterrupt {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let immediate = u8::try_from(instruction_val & 0b1111_1111).unwrap();
        Self {immediate}
    }
}

#[cfg(test)]
mod tests {
    use super::{SoftwareInterrupt, Instruction};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b1011_1110_1100_0011);
        let value = SoftwareInterrupt::from(&instruction);

        let expected_value = SoftwareInterrupt {
            immediate: 0b1100_0011,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
