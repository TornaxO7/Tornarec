use crate::{
    cpus::general::instruction::types::BitState,
    ram::Word,
};

use super::ArmOpcode;

pub fn handle(value: Word) -> ArmOpcode {
    let bit20 = BitState::new(value, 20);

    match bit20 {
        BitState::SET => ArmOpcode::LDM,
        BitState::UNSET => ArmOpcode::STM,
    }
}
