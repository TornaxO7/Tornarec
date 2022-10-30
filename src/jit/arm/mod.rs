use crate::Addr;

use super::JIT;

type ArmInstruction = u32;

impl JIT {
    pub fn compile_arm_block(&mut self, pc: Addr)  {
    }

    pub fn compile_arm_instruction(&mut self, instruction: ArmInstruction) -> bool {
        todo!()
    }
}
