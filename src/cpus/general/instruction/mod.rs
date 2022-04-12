pub mod arm;
pub mod maps;

use crate::{cpus::general::condition_code_flag::ConditionCodeFlag, ram::Address};

pub type InstructionValue = u32;

pub trait Instruction: std::fmt::Debug {
    fn execute(&self);

    fn get_address(&self) -> Address;
}
