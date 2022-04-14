use crate::{cpus::general::instruction::arm::Register, ram::Word};

use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MSRType {
    Immediate { rotate_imm: u8, immediate: u8 },
    Register(Register),
}

impl MSRType {
    pub fn get_immediate(value: Word) -> Self {
        let rotate_imm = u8::try_from((value >> 8) & 0b1111).unwrap();
        let immediate = u8::try_from(value & 0b1111).unwrap();

        Self::Immediate {
            rotate_imm,
            immediate
        }
    }

    pub fn get_register(value: Word) -> Self {
        let register = Register::try_from(value & 0b1111).unwrap();
        Self::Register(register)
    }
}
