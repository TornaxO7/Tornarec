use crate::{ram::Word, cpus::general::instruction::arm::{BitState, types::Register}};

use super::ArmOperand;

use std::convert::TryFrom;

pub fn get_operand(value: Word) -> ArmOperand {
    ArmOperand::LongMultiply {
        s: BitState::new(value, 20),
        rdhi: u8::try_from((value >> 16) & 0b1111).unwrap(),
        rdlo: u8::try_from((value >> 12) & 0b1111).unwrap(),
        rs: Register::new(value, 8, 0b1111),
        rm: Register::new(value, 0, 0b1111),
    }
}
