mod map;

pub use map::get_thumb_instruction;

use crate::ram::{
    Address,
    Word,
};

use super::Instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThumbInstruction {
    pub value: Word,
    pub address: Address,
}

impl Instruction for ThumbInstruction {
    fn execute(&self) {
        todo!()
    }

    fn get_address(&self) -> crate::ram::Address {
        todo!()
    }
}
