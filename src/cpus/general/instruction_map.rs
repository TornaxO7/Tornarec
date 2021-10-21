use crate::cpus::general::{
        instruction::{
            Instruction,
            decoder::{
                ArmDecoder, ThumbDecoder
            },
        },
    };

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionMap {
    Arm(ArmDecoder),
    Thumb(ThumbDecoder),
    Noop,
    Unpredictable,
}

impl InstructionMap {
    pub fn get_arm_instruction(instruction: &Instruction) -> Self {
        Self::Arm(ArmDecoder::from(instruction))
    }

    pub fn get_thumb_instruction(instruction: &Instruction) -> Self {
        Self::Thumb(ThumbDecoder::from(instruction))
    }
}

impl Default for InstructionMap {
    fn default() -> Self {
        Self::Noop
    }
}
