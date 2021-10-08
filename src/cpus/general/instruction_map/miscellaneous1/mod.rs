pub mod checker;
pub mod error;

use checker::Checker;
use error::Miscellaneous1Error;

use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Miscellaneous1 {
    Line1 { op1: u8, rn: u8, rd: u8, rs: u8, op2: u8, rm: u8 }, 
    Line2 { op1: u8, rn: u8, rd: u8, rs: u8, op2: u8, rm: u8 },
    Line3 { r_flag: BitState, rn: u8, rd: u8, rotate_imm: u8, immed_8: u8 }
}

impl From<&Instruction> for Miscellaneous1 {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let rn = u8::try_from((instruction_val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((instruction_val >> 12) & 0b1111).unwrap();

        let rs = u8::try_from((instruction_val >> 8) & 0b1111).unwrap();
        let rotate_imm = rs;

        match Checker::from(instruction) {
            Checker::Line1 => {
                let op1 = u8::try_from((instruction_val >> 21) & 0b11).unwrap();
                let op2 = u8::try_from((instruction_val >> 5) & 0b111).unwrap();
                let rm = u8::try_from(instruction_val & 0b1111).unwrap();
                Self::Line1{op1, rn, rd, rs, op2, rm}
            },
            Checker::Line2 => {
                let op1 = u8::try_from((instruction_val >> 21) & 0b11).unwrap();
                let op2 = u8::try_from((instruction_val >> 5) & 0b11).unwrap();
                let rm = u8::try_from(instruction_val & 0b1111).unwrap();
                Self::Line2{op1, rn, rd, rs, op2, rm}
            },
            Checker::Line3 => {
                let r_flag = BitState::from(instruction_val >> 22);
                let immed_8 = u8::try_from(instruction_val & 0b1111_1111).unwrap();
                Self::Line3{r_flag, rn, rd, rotate_imm, immed_8}
            },
            Checker::Unknown => unreachable!("{}", Miscellaneous1Error::UnknownMiscellaneous(instruction_val)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Miscellaneous1,
        Instruction,
        BitState,
    };

    #[test]
    fn from_line1() {
        let instruction = Instruction::from(0b0000_00010_11_0_1010_0101_1001_000_0_0110);
        let value = Miscellaneous1::from(&instruction);

        let expected_value = Miscellaneous1::Line1 {
            op1: 0b11,
            rn: 0b1010,
            rd: 0b0101,
            rs: 0b1001,
            op2: 0b000,
            rm: 0b0110,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }

    #[test]
    fn test_line2() {
        let instruction = Instruction::from(0b0000_00010_11_0_1010_0101_1001_0_11_1_1111);
        let value = Miscellaneous1::from(&instruction);

        let expected_value = Miscellaneous1::Line2 {
            op1: 0b11,
            rn: 0b1010,
            rd: 0b0101,
            rs: 0b1001,
            op2: 0b11,
            rm: 0b1111,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }

    #[test]
    fn test_line3() {
        let instruction = Instruction::from(0b0000_00110_1_10_1010_0101_1111_11110000);
        let value = Miscellaneous1::from(&instruction);
        
        let expected_value = Miscellaneous1::Line3 {
            r_flag: BitState::Set,
            rn: 0b1010,
            rd: 0b0101,
            rotate_imm: 0b1111,
            immed_8: 0b1111_0000,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }

    #[test]
    #[should_panic]
    fn test_unknown_instruction() {
        let instruction = Instruction::from(0b0000_11010_0101010010_10101010);
        Miscellaneous1::from(&instruction);
    }
}
