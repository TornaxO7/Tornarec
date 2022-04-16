use crate::{
    cpus::general::instruction::arm::BitState,
    ram::Word,
};

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
    let p = BitState::new(value, 24);
    let u = BitState::new(value, 23);
    let n = BitState::new(value, 22);
    let w = BitState::new(value, 21);
}
