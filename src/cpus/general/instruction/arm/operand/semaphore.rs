use crate::{
    cpus::general::instruction::arm::types::{
        sbz,
        Register,
    },
    ram::Word,
};

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
    sbz(value, 8, 0b1111);
    ArmOperand::Semaphore {
        rn: Register::new(value, 16, 0b1111),
        rd: Register::new(value, 12, 0b1111),
        rm: Register::new(value, 0, 0b1111),
    }
}
