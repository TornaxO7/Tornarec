use crate::cpus::general::{
    instruction::decode::DecodeData,
    register::NormalizedRegister,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShiftByImmediate {
    opcode: u8,
    immediate: u8,
    rm: NormalizedRegister,
    rd: NormalizedRegister,
}

impl<'a> From<DecodeData<'a>> for ShiftByImmediate {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 11) & 0b11).unwrap();
        let immediate = u8::try_from((instruction_val >> 6) & 0b11111).unwrap();
        let rm = NormalizedRegister::from((instruction_val >> 3) & 0b111);
        let rd = NormalizedRegister::from(instruction_val & 0b111);
        Self {opcode, immediate, rm, rd}
    }
}

#[cfg(test)]
mod tests {
    use super::{ShiftByImmediate, Instruction, NormalizedRegister};

    use crate::cpus::general::register::RegisterName;

    #[test]
    fn from() {
        let instruction = Instruction::from(0b000_11_10101_101_010);
        let value = ShiftByImmediate::from(&instruction);

        let expected_value = ShiftByImmediate {
            opcode: 0b11,
            immediate: 0b10101,
            rm: NormalizedRegister::from(RegisterName::R5),
            rd: NormalizedRegister::from(RegisterName::R2),
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
