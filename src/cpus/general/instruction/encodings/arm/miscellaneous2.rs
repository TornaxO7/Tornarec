use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Miscellaneous2 {
    op1: u8,
    rn: u8,
    rd: u8,
    rs: u8,
    op2: u8,
    rm: u8,
}

impl<'a> From<DecodeData<'a>> for Miscellaneous2 {
    fn from(data: DecodeData<'a>) -> Self {
        let op1 = u8::try_from((data.instruction.val >> 21) & 0b11).unwrap();
        let rn = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let rs = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let op2 = u8::try_from((data.instruction.val >> 5) & 0b11).unwrap();
        let rm = u8::try_from(data.instruction.val & 0b1111).unwrap();
        Self{op1, rn, rd, rs, op2, rm}
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Miscellaneous2,
        NormalizedRegister,
        DecodeData,
    };

    use crate::{
        cpus::general::{
            register::RegisterName,
            Instruction,
        },
        NintendoDS,
    };

    #[test]
    fn test_from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b0000_00010_11_0_1010_0101_1001_0_11_1_1111);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = Miscellaneous2::from(data);

        let expected_value = Miscellaneous2 {
            op1: 0b11,
            rn: NormalizedRegister::from(RegisterName::R10),
            rd: NormalizedRegister::from(RegisterName::R5),
            rs: NormalizedRegister::from(RegisterName::R9),
            op2: 0b11,
            rm: NormalizedRegister::from(RegisterName::R15),
        };

        assert_eq!(value, expected_value);
    }
}
