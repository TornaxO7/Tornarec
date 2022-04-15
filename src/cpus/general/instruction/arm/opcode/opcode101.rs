
use crate::{ram::Word, cpus::general::instruction::arm::BitState};

use super::ArmOpcode;

pub fn handle(value: Word) -> ArmOpcode {
    let bit24 = BitState::from(((value >> 24) & 0b1) != 0);

    match bit24 {
        true => ArmOpcode::BL,
        false => ArmOpcode::B,
    }
}
