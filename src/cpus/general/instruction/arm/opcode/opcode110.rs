use crate::{ram::Word, cpus::general::instruction::arm::BitState};

use super::ArmOpcode;

pub fn handle(value: Word) -> ArmOpcode {
    let bit20 = BitState::from(((value >> 20) & 0b1) != 0);

    match bit20 {
        true => ArmOpcode::LDC,
        false => ArmOpcode::STC,
    }
}
