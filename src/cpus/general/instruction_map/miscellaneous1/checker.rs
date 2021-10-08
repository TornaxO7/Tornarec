use std::convert::From;

use crate::cpus::general::instruction::Instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Checker {
    Line1,
    Line2,
    Line3,
    Unknown,
}

impl From<&Instruction> for Checker {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        if (instruction_val >> 23) & 0b1_1111 == 0b0_0010
            && (instruction_val >> 20) & 0b1 == 0
            && (instruction_val >> 4) & 0b1 == 0
        {
            Self::Line1
        } else if (instruction_val >> 23) & 0b1_1111 == 0b0_0010
            && (instruction_val >> 20) & 0b1 == 0
            && (instruction_val >> 7) & 0b1 == 0
            && (instruction_val >> 4) & 0b1 == 1
        {
            Self::Line2
        } else if (instruction_val >> 23) & 0b1_1111 == 0b0_0110
            && (instruction_val >> 20) & 0b11 == 0b10
        {
            Self::Line3
        } else {
            Self::Unknown
        }
    }
}
