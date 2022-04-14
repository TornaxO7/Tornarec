use crate::{
    cpus::general::{instruction::arm::{ArmInstruction, opcode::ArmOpcode, operand::ArmOperand}, condition_code_flag::ConditionCodeFlag},
    ram::{
        Address,
        Word,
    },
};

pub fn get_multiply(address: Address, value: Word) -> ArmInstruction {
    ArmInstruction {
        opcode: ArmOpcode::get_multiply(value),
        operand: ArmOperand::get_multiply(value),
        cond: ConditionCodeFlag::from(value),
        address,
    }
}

pub fn get_multiply_long(address: Address, value: Word) -> ArmInstruction {
    ArmInstruction {
        opcode: ArmOpcode::get_multiply_long(value),
        operand: ArmOperand::get_multiply_long(value),
        cond: ConditionCodeFlag::from(value),
        address,
    }
}
