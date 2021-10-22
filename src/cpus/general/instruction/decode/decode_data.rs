use crate::{
    cpus::general::{
        register::Registers,
        Instruction,
    },
    ram::Ram,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeData<'a> {
    pub registers: &'a Registers,
    pub ram: &'a Ram,
    pub instruction: &'a Instruction,
}

impl<'a> DecodeData<'a> {
    pub fn new(registers: &'a Registers, ram: &'a Ram, instruction: &'a Instruction) -> Self {
        Self {
            registers,
            ram,
            instruction,
        }
    }
}
