use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnconditionalBranch {
    offset: u16,
}

impl<'a> From<DecodeData<'a>> for UnconditionalBranch {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

        let offset = u16::try_from(instruction_val & 0b111_1111_1111).unwrap();
        Self { offset }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Instruction,
        UnconditionalBranch,
    };

    #[test]
    fn from() {
        let instruction = Instruction::from(0b11100_111_1010_1010);
        let value = UnconditionalBranch::from(&instruction);

        let expected_value = UnconditionalBranch {
            offset: 0b111_1010_1010,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
