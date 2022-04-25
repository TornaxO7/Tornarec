pub mod opcode;
pub mod operand;
mod types;

use opcode::ArmOpcode;
use operand::ArmOperand;

use crate::{
    cpus::general::condition_code_flag::ConditionCodeFlag,
    ram::{
        Address,
        Word,
    },
};

use super::Instruction;
pub fn get_arm_instruction(_address: Address, _value: Word) -> ArmInstruction {
    todo!();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArmInstruction {
    pub opcode: ArmOpcode,
    pub operand: ArmOperand,
    pub cond: ConditionCodeFlag,
    pub address: Address,
}

impl ArmInstruction {
    pub fn get_condition_flag(&self) -> ConditionCodeFlag {
        self.cond
        // match (&self.val >> 28) & 0b1111 {
        //     0b0000 => ConditionCodeFlag::EQ,
        //     0b0001 => ConditionCodeFlag::NE,
        //     0b0010 => ConditionCodeFlag::CS,
        //     0b0011 => ConditionCodeFlag::CC,
        //     0b0100 => ConditionCodeFlag::MI,
        //     0b0101 => ConditionCodeFlag::PL,
        //     0b0110 => ConditionCodeFlag::VS,
        //     0b0111 => ConditionCodeFlag::VC,
        //     0b1001 => ConditionCodeFlag::HI,
        //     0b1010 => ConditionCodeFlag::LS,
        //     0b1011 => ConditionCodeFlag::GE,
        //     0b1100 => ConditionCodeFlag::LT,
        //     0b1101 => ConditionCodeFlag::LE,
        //     0b1110 => ConditionCodeFlag::AL,
        //     _ => unreachable!("[INSTRUCTION ERROR]: Instruction has unknown
        // condition flag!"), }
    }
}

impl Instruction for ArmInstruction {
    fn execute(&self) {
        todo!()
    }

    fn get_address(&self) -> Address {
        todo!()
    }
}
