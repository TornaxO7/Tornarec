use crate::cpus::general::instruction::Instruction;

pub trait InstructionMapTrait {
    fn is_matching(instruction: &Instruction) -> bool;

    fn match_instruction(instruction: &Instruction) -> Self;
}
