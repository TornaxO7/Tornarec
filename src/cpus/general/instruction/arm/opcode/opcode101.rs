use crate::{
    ram::Word, cpus::general::instruction::types::BitState,
};

use super::ArmOpcode;

pub fn handle(value: Word) -> ArmOpcode {
    let bit24 = BitState::new(value, 24);

    match bit24 {
        BitState::SET => ArmOpcode::BL,
        BitState::UNSET => ArmOpcode::B,
    }
}
