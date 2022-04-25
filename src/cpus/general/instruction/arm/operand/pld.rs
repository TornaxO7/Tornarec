use crate::{
    cpus::general::instruction::{
        arm::types::Register,
        types::BitState,
    },
    ram::Word,
};

use super::{
    load_store::AddressingMode2,
    ArmOperand,
};

pub fn get_operand(value: Word) -> ArmOperand {
    ArmOperand::PLD {
        u: BitState::new(value, 23),
        rn: Register::new(value, 16, 0b1111),
        addr_mode: AddressingMode2::from(value),
    }
}
