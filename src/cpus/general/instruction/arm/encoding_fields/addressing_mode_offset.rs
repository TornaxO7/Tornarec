use crate::cpus::general::instruction::arm::{Register, BitState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode1Offset {
    Immediate {
        rotate_imm: u8,
        immed_8: u8,
    },
    ImmediateShift {
        shift_imm: u8,
        shift: u8,
        rm: u8,
    },
    RegisterShift {
        rs: Register,
        shift: u8,
        rm: Register,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode2Offset {
    ImmediateOffset(u16),
    RegisterOffset(Register),
    ScaledRegisterOffset {
        shift_imm: u8,
        shift: u8,
        rm: Register,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode3Offset {
    Immediate {
        immedH: u8,
        s: BitState,
        h: BitState,
        immedl: u8,
    },
    Register {
        s: BitState,
        h: BitState,
        rm: Register,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode4Offset {
    IncrementAfter,
    IncrementBefore,
    DecrementAfter,
    DecrementBefore,
}

/// Coprocessor addressing mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode5Offset {
    ImmediateOffset,
    ImmediatePreIndexed,
    ImmediatePostIndexed,
    Unindexed,
}
