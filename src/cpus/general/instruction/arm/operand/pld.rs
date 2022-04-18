use crate::{
    cpus::general::instruction::arm::{
        types::Register,
        BitState,
    },
    ram::Word,
};

use super::{ArmOperand, load_store::AddressingMode2};

pub fn get_operand(value: Word) -> ArmOperand {
    ArmOperand::PLD {
        u: BitState::new(value, 23),
        rn: Register::new(value, 16, 0b1111),
        addr_mode: AddressingMode2::from(value),
    }
}
