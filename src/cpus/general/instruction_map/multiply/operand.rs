use crate::cpus::general::{
    register::types::RegisterIndex,
    bit_state::BitState,
    instruction_map::encoding_types::field::immed_8::Immed8,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiplyOperand {
    MLA {
        s_flag: BitState,
        rd: RegisterIndex,
        rn: RegisterIndex,
        rs: RegisterIndex,
        rm: RegisterIndex,
    },
    MUL {
        s_flag: BitState,
        rd: RegisterIndex,
        rs: RegisterIndex,
        rm: RegisterIndex,
    },
    SMLAL {
        s_flag: BitState,
        rdhi: Immed8,
        rdlo: Immed8,
        rs: RegisterIndex,
        rm: RegisterIndex,
    },
    SMULL {
        s_flag: BitState,
        rdhi: Immed8,
        rdlo: Immed8,
        rs: RegisterIndex,
        rm: RegisterIndex,
    },
    UMLAL {
        s_flag: BitState,
        rdhi: Immed8,
        rdlo: Immed8,
        rs: RegisterIndex,
        rm: RegisterIndex,
    },
    UMULL {
        s_flag: BitState,
        rdhi: Immed8,
        rdlo: Immed8,
        rs: RegisterIndex,
        rm: RegisterIndex,
    },
}

impl MultiplyOperand {
    pub const MLA_CODE: u32 = 0b000_0001;
    pub const MUL_CODE: u32 = 0b000_0000;
    pub const SMLAL_CODE: u32 = 0b000_0111;
    pub const SMULL_CODE: u32 = 0b000_0110;
    pub const UMLAL_CODE: u32 = 0b000_0101;
    pub const UMULL_CODE: u32 = 0b000_0100;
}
