pub mod operand;
pub mod addressing_mode;
pub mod error;

pub use operand::MiscellaneousOperand;
pub use addressing_mode::AddressingMode;
pub use error::MiscellaneousError;

use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Miscellaneous {
    operand: MiscellaneousOperand,
    rn: u8,
    rd: u8,
    s_flag: BitState,
    h_flag: BitState,
    addressing_mode: AddressingMode,
}

impl From<&Instruction> for Miscellaneous {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let rn = (instruction_val >> 16) & 0b1111;
        let rd = (instruction_val >> 12) & 0b1111;
        let s_flag = BitState::from(instruction_val >> 6);
        let h_flag = BitState::from(instruction_val >> 5);
        let addressing_mode = AddressingMode::from(instruction);

        Self {
            rn,
            rd,
            s_flag,
            h_flag,
            addressing_mode,
        }
    }
}
