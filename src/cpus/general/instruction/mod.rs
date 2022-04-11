pub mod arm;

use crate::cpus::general::condition_code_flag::ConditionCodeFlag;

pub type InstructionValue = u32;

pub trait Instruction {
    type InstructionValue;

    fn get_condition_flag() -> ConditionCodeFlag {
        match (Self::InstructionValue >> 28) & 0b1111 {
            0b0000 => ConditionCodeFlag::EQ,
            0b0001 => ConditionCodeFlag::NE,
            0b0010 => ConditionCodeFlag::CS,
            0b0011 => ConditionCodeFlag::CC,
            0b0100 => ConditionCodeFlag::MI,
            0b0101 => ConditionCodeFlag::PL,
            0b0110 => ConditionCodeFlag::VS,
            0b0111 => ConditionCodeFlag::VC,
            0b1001 => ConditionCodeFlag::HI,
            0b1010 => ConditionCodeFlag::LS,
            0b1011 => ConditionCodeFlag::GE,
            0b1100 => ConditionCodeFlag::LT,
            0b1101 => ConditionCodeFlag::LE,
            0b1110 => ConditionCodeFlag::AL,
            _ => unreachable!("[INSTRUCTION ERROR]: Instruction has unknown condition flag!"),
        }
    }

    fn execute();
}
