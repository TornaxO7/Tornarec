use crate::{
    cpus::general::instruction::arm::BitState,
    ram::Word,
};

use super::ArmOpcode;

pub fn handle(value: Word) -> ArmOpcode {
    let bit27_20 = (value >> 20) & 0b1111_1111;
    match bit27_20 {
        0b1100_0100 => return ArmOpcode::MCRR,
        0b1100_0101 => return ArmOpcode::MRRC,
        _ => (),
    };

    let bit20 = BitState::from(((value >> 20) & 0b1) != 0);
    match bit20 {
        true => ArmOpcode::LDC,
        false => ArmOpcode::STC,
    }
}
