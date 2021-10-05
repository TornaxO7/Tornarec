pub mod operand;
pub mod error;

use operand::LoadAndStoreMultipleOperand;
use error::LoadAndStoreMultipleError;

use crate::cpus::general::{
    bit_state::BitState,
    register::types::RegisterIndex,
    instruction::Instruction,
    instruction_map::InstructionMapTrait,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadAndStoreMultiple {
    p_flag: BitState,
    u_flag: BitState,
    s_flag: BitState,
    w_flag: BitState,
    l_flag: BitState,
    rn: RegisterIndex,
    register_list: [BitState; 15],
}

impl InstructionMapTrait for LoadAndStoreMultiple {
    type Operand = LoadAndStoreMultipleOperand;

    fn is_matching(instruction: &Instruction) -> bool {
        (instruction.get_value_as_u32() >> 25) & 0b111 == 0b100
    }

    fn get_operand(&self) -> Self::Operand {
        // differ between LDM and STM first
        if self.l_flag.is_set() {
            match (self.s_flag, self.w_flag, self.register_list[15]) {
                (BitState::Unset, _, _) => LoadAndStoreMultipleOperand::LDM1,
                (BitState::Set, BitState::Unset, BitState::Unset) => LoadAndStoreMultipleOperand::LDM2,
                (BitState::Set, _, BitState::Set) => LoadAndStoreMultipleOperand::LDM3,
                _other => unreachable!("{}", LoadAndStoreMultipleError::UnknownLDMInstruction(self.clone())),
            }
        } else {
            match (self.s_flag, self.w_flag) {
                (BitState::Unset, _) => LoadAndStoreMultipleOperand::STM1,
                (BitState::Set, BitState::Unset) => LoadAndStoreMultipleOperand::STM2,
                _ => unreachable!("{}", LoadAndStoreMultipleError::UnknownSTMInstruction(self.clone())),
            }
        }
    }
}
