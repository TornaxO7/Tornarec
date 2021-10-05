pub mod operand;
pub mod error;

pub use operand::LoadAndStoreOperand;

use crate::cpus::general::{
    instruction::Instruction,
    instruction_map::{
        encoding_types::AddressingMode,
        InstructionMapTrait,
    },
    bit_state::BitState,
    register::types::RegisterIndex,
};

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadAndStore {
    p_flag: BitState,
    u_flag: BitState,
    b_flag: BitState,
    w_flag: BitState,
    l_flag: BitState,
    rn: RegisterIndex,
    rd: RegisterIndex,
    addressing_mode: AddressingMode,
}

impl InstructionMapTrait for LoadAndStore {
    type Operand = LoadAndStoreOperand;

    fn is_matching(instruction: &Instruction) -> bool {
        (instruction.get_value_as_u32() >> 26) & 0b11 == 0b01
    }

    fn get_operand(&self) -> Self::Operand {
        match (self.p_flag, self.u_flag, self.b_flag, self.w_flag, self.l_flag) {
            (_, _, BitState::Unset, _, BitState::Set) => LoadAndStoreOperand::LDR,
            (_, _, BitState::Set, _, BitState::Set) => LoadAndStoreOperand::LDRB,
            (BitState::Unset, _, BitState::Set, BitState::Set, BitState::Set) => LoadAndStoreOperand::LDRBT,
            (_, _, _, _, BitSt)
        }
    }
}
