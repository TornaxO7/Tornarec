use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoftwareBreakpoint {
    immediate: u8,
}

impl<'a> From<DecodeData<'a>> for SoftwareBreakpoint {
    fn from(data: DecodeData<'a>) -> Self {
        let immediate = u8::try_from(data.instruction.val & 0b1111_1111).unwrap();
        Self { immediate }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DecodeData,
        SoftwareBreakpoint,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b1011_1110_1100_0011,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = SoftwareBreakpoint::from(data);

        let expected_value = SoftwareBreakpoint {
            immediate: 0b1100_0011,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
