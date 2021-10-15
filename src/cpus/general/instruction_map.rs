use crate::cpus::general::{
    Instruction,
    instruction_state::{
        ArmInstruction, ThumbInstruction
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionMap {
    Arm(ArmInstruction),
    Thumb(ThumbInstruction),
    Noop,
}

impl InstructionMap {
    pub fn get_arm_instruction(instruction: &Instruction) -> Self {
        Self::Arm(ArmInstruction::from(instruction))
    }

    pub fn get_thumb_instruction(instruction: &Instruction) -> Self {
        Self::Thumb(ThumbInstruction::from(instruction))
    }
}

impl Default for InstructionMap {
    fn default() -> Self {
        Self::Noop
    }
}
