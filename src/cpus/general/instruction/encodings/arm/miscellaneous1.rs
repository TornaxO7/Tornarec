use crate::cpus::general::{
    instruction::decode::DecodeData,
    register::NormalizedRegister,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Miscellaneous1 {
    op1: u8,
    rn: NormalizedRegister,
    rd: NormalizedRegister,
    rs: NormalizedRegister,
    op2: u8,
    rm: NormalizedRegister,
    // Line3 { r_flag: BitState, rn: u8, rd: u8, rotate_imm: u8, immed_8: u8 }
}

impl<'a> From<DecodeData<'a>> for Miscellaneous1 {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

        let rn = NormalizedRegister::from((instruction_val >> 16) & 0b1111);
        let rd = NormalizedRegister::from((instruction_val >> 12) & 0b1111);
        let rs = NormalizedRegister::from((instruction_val >> 8) & 0b1111);
        let op1 = u8::try_from((instruction_val >> 21) & 0b11).unwrap();
        let op2 = u8::try_from((instruction_val >> 5) & 0b111).unwrap();
        let rm = NormalizedRegister::from(instruction_val & 0b1111);
        Self{op1, rn, rd, rs, op2, rm}
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Miscellaneous1,
        NormalizedRegister
    };

    use crate::cpus::general::register::RegisterName;

    #[test]
    fn from() {
        let instruction = Instruction::from(0b0000_00010_11_0_1010_0101_1001_000_0_0110);
        let value = Miscellaneous1::from(&instruction);

        let expected_value = Miscellaneous1 {
            op1: 0b11,
            rn: NormalizedRegister::from(RegisterName::R10),
            rd: NormalizedRegister::from(RegisterName::R5),
            rs: NormalizedRegister::from(RegisterName::R9),
            op2: 0b000,
            rm: NormalizedRegister::from(RegisterName::R6),
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
