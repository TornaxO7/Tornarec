use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingImmediateShift {
    opcode: u8,
    s_flag: BitState,
    rn: u8,
    rd: u8,
    shift_amount: u8,
    shift: u8,
    rm: u8,
}

impl From<&Instruction> for DataProcessingImmediateShift {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 21) & 0b1111).unwrap();
        let s_flag = BitState::from(instruction_val >> 20);
        let rn = u8::try_from((instruction_val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((instruction_val >> 12) & 0b1111).unwrap();
        let shift_amount = u8::try_from((instruction_val >> 7) & 0b1_1111).unwrap();
        let shift = u8::try_from((instruction_val >> 5) & 0b11).unwrap();
        let rm = u8::try_from(instruction_val & 0b1111).unwrap();

        Self{opcode, s_flag, rn, rd, shift_amount, shift, rm}
    }
}
