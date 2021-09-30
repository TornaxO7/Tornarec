use core::convert::From;

use crate::ram::data_types::DataType;
use crate::cpus::general::register::cpsr::ConditionCodeFlag;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Instruction(u32);

impl Instruction {
    pub fn get_value_as_u32(&self) -> u32 {
        self.0
    }

    pub fn get_condition_code_flag(&self) -> ConditionCodeFlag {
        match (self.0.clone() >> 28) & 0b1111 {
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
            _other => unreachable!("[INSTRUCTION ERROR]: Instruction has unknown condition flag!"),
        }
    }
}

impl From<u32> for Instruction {
    fn from(num: u32) -> Self {
        Self(num)
    }
}

impl From<DataType> for Instruction {
    fn from(data_type: DataType) -> Self {
        Self(data_type.get_value_as_u32())
    }
}
