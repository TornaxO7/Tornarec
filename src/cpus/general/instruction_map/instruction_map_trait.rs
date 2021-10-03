use crate::cpus::general::instruction::Instruction;

pub trait InstructionMapTrait {

    type Operand;

    fn is_matching(instruction: &Instruction) -> bool;

    fn get_operand(&self) -> Self::Operand;
}
