use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShiftByImmediate {
    opcode: u8,
    immediate: u8,
    rm: u8,
    rd: u8,
}

impl<'a> From<DecodeData<'a>> for ShiftByImmediate {
    fn from(data: DecodeData<'a>) -> Self {
        let opcode = u8::try_from((data.instruction.val >> 11) & 0b11).unwrap();
        let immediate = u8::try_from((data.instruction.val >> 6) & 0b11111).unwrap();
        let rm = u8::try_from((data.instruction.val >> 3) & 0b111).unwrap();
        let rd = u8::try_from(data.instruction.val & 0b111).unwrap();
        Self {
            opcode,
            immediate,
            rm,
            rd,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DecodeData,
        ShiftByImmediate,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b000_11_10101_101_010,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = ShiftByImmediate::from(data);

        let expected_value = ShiftByImmediate {
            opcode: 0b11,
            immediate: 0b10101,
            rm: 0b0101,
            rd: 0b0010,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
