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
