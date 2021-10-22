use crate::cpus::general::instruction::decode::{
    ArmDecode, ThumbDecode, DecodeData,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionMap {
    Arm(ArmDecode),
    Thumb(ThumbDecode),
    Noop,
    Unpredictable,
}

impl InstructionMap {
    pub fn get_arm_instruction(decode_data: DecodeData) -> Self {
        Self::Arm(ArmDecode::from(decode_data))
    }

    pub fn get_thumb_instruction(decode_data: DecodeData) -> Self {
        Self::Thumb(ThumbDecode::from(decode_data))
    }
}

impl Default for InstructionMap {
    fn default() -> Self {
        Self::Noop
    }
}
