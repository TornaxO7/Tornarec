use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoftwareBreakpoint {
    immed_12: u16,
    immed: u8,
}

impl<'a> From<DecodeData<'a>> for SoftwareBreakpoint {
    fn from(data: DecodeData<'a>) -> Self {
        let immed_12 = u16::try_from((data.instruction.val >> 8) & 0b1111_1111_1111).unwrap();
        let immed = u8::try_from(data.instruction.val & 0b1111).unwrap();

        Self { immed_12, immed }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    use super::{
        DecodeData,
        SoftwareBreakpoint,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let data = {
            let instruction = Instruction {
                val: 0b0000_00010_010_1111_1110_1100_0111_1001,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = SoftwareBreakpoint::from(data);
        let expected_value = SoftwareBreakpoint {
            immed_12: 0b1111_1110_1100,
            immed: 0b1001,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
        );
    }
}
