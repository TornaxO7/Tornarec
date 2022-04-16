use crate::{
    cpus::general::instruction::arm::BitState,
    ram::Word,
};

use super::ArmOpcode;

pub fn handle(value: Word) -> ArmOpcode {
    let bit24 = BitState::new(value, 24);

    match bit24 {
        BitState::SET => ArmOpcode::BL,
        BitState::UNSET => ArmOpcode::B,
    }
}
