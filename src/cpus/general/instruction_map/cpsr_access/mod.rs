pub mod error;
pub mod msr_operand;

pub use error::CpsrAccessError;
pub use msr_operand::MsrOperand;

use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
    register::types::RegisterIndex,
};

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpsrAccess {
    MRS {
        r_flag: BitState,
        rd: u8,
    },
    MSR(MsrOperand),
}

impl CpsrAccess {
    pub fn is_msr(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();

        (instruction_val >> 23) & 0b11111 == 0b00110
            && (instruction_val >> 20) & 0b11 == 0b10
    }

    pub fn is_mrs(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();
        
        (instruction_val >> 23) & 0b11111 == 0b00010
            && (instruction_val >> 20) & 0b11 == 0b00
    }
}

impl From<&Instruction> for CpsrAccess {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        if Self::is_msr(instruction) {
            let r_flag = BitState::from(instruction_val >> 22);
            let rd = RegisterIndex::from((instruction_val >> 12) & 0b1111);

            if (instruction_val >> 16) & 0b1111 != 0b1111 {
                panic!("{}", CpsrAccessError::SBOConflict(16, 19, instruction_val));
            }
            if instruction_val & 0b111_1111_1111 != 0b000_0000_0000 {
                panic!("{}", CpsrAccessError::SBZConflict(0, 11, instruction_val));
            }

            Self::MRS {
                r_flag,
                rd,
            }
        } else if Self::is_mrs(instruction) {
            Self::MSR(MsrOperand::from(instruction))
        } else {
            unreachable!("{}", CpsrAccessError::UnknownOperand(instruction_val));
        }
    }
}
