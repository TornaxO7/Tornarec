use crate::cpus::general::{
    register::Registers,
    Instruction,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeData<'a> {
    pub instruction: Instruction,
    pub registers: &'a Registers,
}

impl<'a> DecodeData<'a> {
    pub fn new(instruction: Instruction, registers: &'a Registers) -> Self {
        Self {
            instruction,
            registers,
        }
    }
}
