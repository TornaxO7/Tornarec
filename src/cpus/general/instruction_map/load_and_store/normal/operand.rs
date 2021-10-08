use std::convert::From;

use crate::cpus::general::{
    instruction::Instruction,
    bit_state::BitState,
    instruction_map::load_and_store::normal::error::NormalError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalOperand { LDR, LDRB, LDRBT, LDRT, STR, STRB, STRBT, STRT }

impl From<&Instruction> for NormalOperand {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let p_flag = BitState::from(instruction_val >> 24);
        let u_flag = BitState::from(instruction_val >> 23);
        let b_flag = BitState::from(instruction_val >> 22);
        let w_flag = BitState::from(instruction_val >> 21);
        let l_flag = BitState::from(instruction_val >> 20);

        match (p_flag, u_flag, b_flag, w_flag, l_flag) {
            (_, _, 0, _, 1) => Self::LDR,
            (_, _, 1, _, 1) => Self::LDRB,
            (0, _, 1, 1, 1) => Self::LDRBT,
            (0, _, 0, 1, 1) => Self::LDRT,
            (_, _, 0, _, 0) => Self::STR,
            (_, _, 1, _, 0) => Self::STRB,
            (0, _, 1, 1, 0) => Self::STRBT,
            (0, _, 0, 1, 0) => Self::STRT,
            _other => unreachable!("{}", NormalError::UnknownOperand(instruction_val)),
        }
    }
}
