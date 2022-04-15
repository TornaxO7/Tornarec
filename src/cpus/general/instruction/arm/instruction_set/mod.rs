//! This module contains the instructions listed in Figure A3-1
mod instruction000;
mod instruction001;
mod instruction010;
mod instruction011;
mod instruction100;
mod instruction101;
mod instruction110;
mod instruction111;

use instruction000::handle000;
use instruction001::handle001;
use instruction010::handle010;
use instruction011::handle011;
use instruction100::handle100;
use instruction101::handle101;
use instruction110::handle110;
use instruction111::handle111;

use crate::{
    cpus::general::condition_code_flag::ConditionCodeFlag,
    ram::{
        Address,
        Word,
    },
};

use super::{
    opcode::ArmOpcode,
    operand::ArmOperand,
    ArmInstruction,
};

pub fn get_arm_instruction(address: Address, value: Word) -> ArmInstruction {
    let cond = ConditionCodeFlag::from(value);

    let opcode = ArmOpcode::from(value);
    let operand = ArmOperand::parse_operand(opcode, value);

    ArmInstruction {
        opcode,
        operand,
        cond,
        address,
    }
}
