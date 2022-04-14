pub mod arm;
pub mod thumb;

use crate::ram::Address;

pub type InstructionValue = u32;

pub trait Instruction: std::fmt::Debug {
    fn execute(&self);

    fn get_address(&self) -> Address;
}
