use crate::{ram::Word, cpus::general::instruction::arm::{BitState, types::Register}};

use super::{ArmOperand, load_store_word_byte::AddressingMode2};

pub fn get_operand(value: Word) -> ArmOperand {
    ArmOperand::PLD {
        u: BitState::new(value, 23),
        rn: Register::new(value, 16, 0b1111),
        addr_mode: AddressingMode2::from(value),
    }
}
