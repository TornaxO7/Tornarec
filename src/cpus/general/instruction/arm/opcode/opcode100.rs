use crate::{
    cpus::general::instruction::arm::BitState,
    ram::Word,
};

use super::ArmOpcode;

pub fn handle(value: Word) -> ArmOpcode {
    let bit20 = BitState::from(((value >> 20) & 0b1) != 0);

    match bit20 {
        true => ArmOpcode::LDM,
        false => ArmOpcode::STM,
    }
}
