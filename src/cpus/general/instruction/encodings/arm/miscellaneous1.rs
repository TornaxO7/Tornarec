use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Miscellaneous1 {
    op1: u8,
    rn: u8,
    rd: u8,
    rs: u8,
    op2: u8,
    rm: u8,
    // Line3 { r_flag: BitState, rn: u8, rd: u8, rotate_imm: u8, immed_8: u8 }
}

impl<'a> From<DecodeData<'a>> for Miscellaneous1 {
    fn from(data: DecodeData<'a>) -> Self {
        let rn = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let rs = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let op1 = u8::try_from((data.instruction.val >> 21) & 0b11).unwrap();
        let op2 = u8::try_from((data.instruction.val >> 5) & 0b111).unwrap();
        let rm = u8::try_from(data.instruction.val & 0b1111).unwrap();
        Self {
            op1,
            rn,
            rd,
            rs,
            op2,
            rm,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DecodeData,
        Miscellaneous1,
        NormalizedRegister,
    };

    use crate::{
        cpus::general::{
            register::RegisterName,
            Instruction,
        },
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b0000_00010_11_0_1010_0101_1001_000_0_0110);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = Miscellaneous1::from(data);

        let expected_value = Miscellaneous1 {
            op1: 0b11,
            rn: NormalizedRegister::from(RegisterName::R10),
            rd: NormalizedRegister::from(RegisterName::R5),
            rs: NormalizedRegister::from(RegisterName::R9),
            op2: 0b000,
            rm: NormalizedRegister::from(RegisterName::R6),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
