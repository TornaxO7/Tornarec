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

impl From<&Instruction> for LoadAndStore {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let p_flag = BitState::from(instruction_val >> 24);
        let u_flag = BitState::from(instruction_val >> 23);
        let b_flag = BitState::from(instruction_val >> 22);
        let w_flag = BitState::from(instruction_val >> 21);
        let l_flag = BitState::from(instruction_val >> 20);
        let rn = RegisterIndex::from(instruction_val >> 16);
        let rd = RegisterIndex::from(instruction_val >> 12);
        let addressing_mode = AddressingMode::from(instruction);

        Self {
            p_flag,
            u_flag,
            b_flag,
            w_flag,
            l_flag,
            rn,
            rd,
            addressing_mode,
        }
    }
}
