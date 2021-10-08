use std::convert::{From, TryFrom};

use crate::cpus::general::instruction::Instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Miscellaneous2 {
    op1: u8,
    rn: u8,
    rd: u8,
    rs: u8,
    op2: u8,
    rm: u8,
}

impl From<&Instruction> for Miscellaneous2 {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let op1 = u8::try_from((instruction_val >> 21) & 0b11).unwrap();
        let rn = u8::try_from((instruction_val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((instruction_val >> 12) & 0b1111).unwrap();
        let rs = u8::try_from((instruction_val >> 8) & 0b1111).unwrap();
        let op2 = u8::try_from((instruction_val >> 5) & 0b11).unwrap();
        let rm = u8::try_from(instruction_val & 0b1111).unwrap();
        Self{op1, rn, rd, rs, op2, rm}
    }
}

#[cfg(test)]
mod tests {
    use super::{Miscellaneous2, Instruction};

    #[test]
    fn test_from() {
        let instruction = Instruction::from(0b0000_00010_11_0_1010_0101_1001_0_11_1_1111);
        let value = Miscellaneous2::from(&instruction);

        let expected_value = Miscellaneous2 {
            op1: 0b11,
            rn: 0b1010,
            rd: 0b0101,
            rs: 0b1001,
            op2: 0b11,
            rm: 0b1111,
        };

        assert_eq!(value, expected_value);
    }
}
