use crate::cpus::general::instruction::decode::{
    arm::ArmDecode,
    DecodeData,
    ThumbDecode,
};

use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionMap {
    Arm(ArmDecode),
    Thumb(ThumbDecode),
    Noop,
    Unpredictable,
    Undefined,
}

impl<'a> InstructionMap {
    pub fn get_arm_instruction(data: DecodeData<'a>) -> Self {
        match ArmDecode::try_from(data) {
            Ok(decode) => Self::Arm(decode),
            Err(_) => Self::Undefined,
        }
    }

    pub fn get_thumb_instruction(decode_data: DecodeData<'a>) -> Self {
        Self::Thumb(ThumbDecode::from(decode_data))
    }
}

impl Default for InstructionMap {
    fn default() -> Self {
        Self::Noop
    }
}
