use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoftwareInterrupt {
    immediate: u8
}

impl<'a> From<DecodeData<'a>> for SoftwareInterrupt {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

        let immediate = u8::try_from(instruction_val & 0b1111_1111).unwrap();
        Self {immediate}
    }
}

#[cfg(test)]
mod tests {
    use super::{SoftwareInterrupt, Instruction};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b1101_1111_1100_1000);
        let value = SoftwareInterrupt::from(&instruction);

        let expected_value = SoftwareInterrupt {
            immediate: 0b1100_1000,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
