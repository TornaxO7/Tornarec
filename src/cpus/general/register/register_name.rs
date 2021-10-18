#![allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterName {
    // general purpose registers
    R0, R1, R2, R3, R4, R5, R6, R7,

    // banked fiq registers
    R8,  R8Fiq,
    R9,  R9Fiq,
    R10, R10Fiq,
    R11, R11Fiq,
    R12, R12Fiq,

    // Format: <Register Index>, <Aliasname>
    R13, Sp,
    R13Svc, SpSvc,
    R13Abt, SpAbt,
    R13Und, SpUnd,
    R13Irq, SpIrq,
    R13Fiq, SpFiq,

    // Format: <Register Index>, <Aliasname>
    R14, Lr,
    R14Svc, LrSvc,
    R14Abt, LrAbt,
    R14Und, LrUnd,
    R14Irq, LrIrq,
    R14Fiq, LrFiq,

    // Format: <Register Index>, <Aliasname>
    R15, Pc,

    Cpsr,
    SpsrSvc, SpsrAbt, SpsrUnd, SpsrIrq, SpsrFiq
}

impl From<u32> for RegisterName {
    fn from(num: u32) -> Self {
        match num {
            0b0000 => Self::R0, 
            0b0001 => Self::R1, 
            0b0010 => Self::R2, 
            0b0011 => Self::R3, 
            0b0100 => Self::R4, 
            0b0101 => Self::R5, 
            0b0110 => Self::R6, 
            0b0111 => Self::R7, 
            0b1000 => Self::R8, 
            0b1001 => Self::R9, 
            0b1010 => Self::R10, 
            0b1011 => Self::R11, 
            0b1100 => Self::R12, 
            0b1101 => Self::R13, 
            0b1110 => Self::R14, 
            0b1111 => Self::R15, 
            _other => unreachable!("[Register Name Error]: '{:b}' is an unknown register.", _other),
        }
    }
}
