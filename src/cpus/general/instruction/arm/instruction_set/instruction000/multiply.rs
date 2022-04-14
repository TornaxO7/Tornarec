use crate::{
    cpus::general::{
        condition_code_flag::ConditionCodeFlag,
        instruction::arm::{
            opcode::ArmOpcode,
            operand::ArmOperand,
            ArmInstruction,
            BitState,
        },
    },
    ram::{
        Address,
        Word,
    },
};

pub fn get_multiply_instruction(address: Address, value: Word) -> ArmInstruction {
    let bit23 = BitState::from(((value >> 23) & 0b1) != 0);

    if bit23 {
        get_multiply_long(address, value)
    } else {
        get_multiply(address, value)
    }
}

pub fn is_multiply_instruction(value: Word) -> bool {
    let bit31_28 = (value >> 28) & 0b1111;
    let bit27_24 = (value >> 24) & 0b1111;
    let bit7_4 = (value >> 4) & 01111;

    bit31_28 != 0b1111 && bit27_24 == 0b0000 && bit7_4 == 0b1001
}

fn get_multiply(address: Address, value: Word) -> ArmInstruction {
    ArmInstruction {
        opcode: ArmOpcode::get_multiply(value),
        operand: ArmOperand::get_multiply(value),
        cond: ConditionCodeFlag::from(value),
        address,
    }
}

fn get_multiply_long(address: Address, value: Word) -> ArmInstruction {
    ArmInstruction {
        opcode: ArmOpcode::get_multiply_long(value),
        operand: ArmOperand::get_multiply_long(value),
        cond: ConditionCodeFlag::from(value),
        address,
    }
}
