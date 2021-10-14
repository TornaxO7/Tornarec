use crate::cpus::general::instruction::Instruction;

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Multiplies {
    op1: u8,
    rn: u8,
    rd: u8,
    rs: u8,
    rm: u8,
}

impl From<&Instruction> for Multiplies {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let op1 = u8::try_from((instruction_val >> 20) & 0b1111).unwrap();
        let rn = u8::try_from((instruction_val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((instruction_val >> 12) & 0b1111).unwrap();
        let rs = u8::try_from((instruction_val >> 8) & 0b1111).unwrap();
        let rm = u8::try_from(instruction_val & 0b1111).unwrap();
        Self{op1, rn, rd, rs, rm}
    }
}

#[cfg(test)]
mod tests {
    use super::{Multiplies, Instruction};

    #[test]
    fn test_from() {
        let instruction = Instruction::from(0b0000_0000_1111_1100_0011_1110_1001_0110);
        let value = Multiplies::from(&instruction);

        let expected_value = Multiplies {
            op1: 0b1111,
            rn: 0b1100,
            rd: 0b0011,
            rs: 0b1110,
            rm: 0b0110,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
