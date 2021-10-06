pub mod operand;
pub mod error;

pub use error::MultiplyError;

use crate::cpus::general::{
    instruction::Instruction,
    bit_state::BitState,
};

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Multiply {
    MLA {
        s_flag: BitState,
        rd: u8,
        rn: u8,
        rs: u8,
        rm: u8,
    },
    MUL {
        s_flag: BitState,
        rd: u8,
        rs: u8,
        rm: u8,
    },
    SMLAL {
        s_flag: BitState,
        rdhi: u8,
        rdlo: u8,
        rs: u8,
        rm: u8,
    },
    SMULL {
        s_flag: BitState,
        rdhi: u8,
        rdlo: u8,
        rs: u8,
        rm: u8,
    },
    UMLAL {
        s_flag: BitState,
        rdhi: u8,
        rdlo: u8,
        rs: u8,
        rm: u8,
    },
    UMULL {
        s_flag: BitState,
        rdhi: u8,
        rdlo: u8,
        rs: u8,
        rm: u8,
    },
}

impl Multiply {
    pub const MLA_CODE: u32 = 0b000_0001;
    pub const MUL_CODE: u32 = 0b000_0000;
    pub const SMLAL_CODE: u32 = 0b000_0111;
    pub const SMULL_CODE: u32 = 0b000_0110;
    pub const UMLAL_CODE: u32 = 0b000_0101;
    pub const UMULL_CODE: u32 = 0b000_0100;
}

impl From<&Instruction> for Multiply {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let s_flag = BitState::from(instruction_val >> 20);

        let rd = (instruction_val >> 16) & 0b1111;
        let rdhi = rd;

        let rn = (instruction_val >> 12) & 0b1111;
        let rdlo = rn;

        let rs = (instruction_val >> 8) & 0b1111;
        let rm = instruction_val & 0b1111;

        match (instruction_val >> 21) & 0b111_1111 {
            Self::MLA_CODE => Self::MLA{s_flag, rd, rn, rs, rm},
            Self::MUL_CODE => {
                if rn != 0b0000 {
                    panic!("{}", MultiplyError::SBZConflict(instruction_val));
                }
                Self::MUL{s_flag, rd, rs, rm};
            },
            Self::SMLAL_CODE => Self::SMLAL{s_flag, rdhi, rdlo, rs, rm},
            Self::SMULL_CODE => Self::SMULL{s_flag, rdhi, rdlo, rs, rm},
            Self::UMLAL_CODE => Self::UMLAL{s_flag, rdhi, rdlo, rs, rm},
            Self::UMULL_CODE => Self::UMULL{s_flag, rdhi, rdlo, rs, rm},
        }
    }
}
