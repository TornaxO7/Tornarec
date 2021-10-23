use crate::cpus::general::Instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeData {
    pub instruction: Instruction,
    pub next_instruction: Instruction,
}

impl<'a> DecodeData {
    pub fn new(instruction: Instruction, next_instruction: Instruction) -> Self {
        Self {
            instruction,
            next_instruction,
        }
    }
}
