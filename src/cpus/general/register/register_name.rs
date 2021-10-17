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
