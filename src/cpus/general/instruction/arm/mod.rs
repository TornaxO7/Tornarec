pub mod opcode;
pub mod operand;
pub mod encoding_fields;

use opcode::ArmOpcode;
use operand::ArmOperand;

use crate::cpus::general::condition_code_flag::ConditionCodeFlag;

pub type BitState = bool;
pub type Register = u8;

/// coprocessor register
pub type CPRegister = u8;
/// coprocessor number
pub type CPNum = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArmInstruction {
    pub opcode: ArmOpcode,
    pub operand: ArmOperand,
    cond: ConditionCodeFlag,
}
