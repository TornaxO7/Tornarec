use std::convert::From;

use crate::cpus::general::{
    instruction::Instruction,
    bit_state::BitState,
};

use super::MiscellaneousError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiscellaneousOperand { LDRD, LDRH, LDRSB, LDRSH, STRD, STRH }

impl From<&Instruction> for MiscellaneousOperand {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let p_flag = BitState::from(instruction_val >> 24);
        let u_flag = BitState::from(instruction_val >> 23);
        let i_flag = BitState::from(instruction_val >> 22);
        let w_flag = BitState::from(instruction_val >> 21);
        let l_flag = BitState::from(instruction_val >> 20);

        let s_flag = BitState::from(instruction >> 6);
        let h_flag = BitState::from(instruction >> 5);

        match (p_flag, u_flag, i_flag, w_flag, l_flag, s_flag, h_flag) {
            (_, _, _, _, 0, 1, 0) => Self::LDRD,
            (_, _, _, _, 1, 0, 1) => Self::LDRH,
            (_, _, _, _, 1, 1, 0) => Self::LDRSB,
            (_, _, _, _, 1, 1, 1) => Self::LDRSH,
            (_, _, _, _, 0, 1, 1) => Self::STRD,
            (_, _, _, _, 0, 0, 1) => Self::STRH,
            _other => unreachable!("{}", MiscellaneousError::UnknownOperand(instruction_val)),
        }
    }
}
