use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSubtractCompareMoveImmediate {
    opcode: u8,
    rd_rn: u8,
    immediate: u8,
}

impl<'a> From<DecodeData<'a>> for AddSubtractCompareMoveImmediate {
    fn from(data: DecodeData<'a>) -> Self {
        let opcode = u8::try_from((data.instruction.val >> 11) & 0b11).unwrap();
        let rd_rn = u8::try_from((data.instruction.val >> 8) & 0b111).unwrap();
        let immediate = u8::try_from(data.instruction.val & 0b1111_1111).unwrap();
        Self {opcode, rd_rn, immediate}
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AddSubtractCompareMoveImmediate,
        DecodeData,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b001_11_110_1010_0101,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = AddSubtractCompareMoveImmediate::from(data);

        let expected_value = AddSubtractCompareMoveImmediate {
            opcode: 0b11,
            rd_rn: 0b0110,
            immediate: 0b1010_0101,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
