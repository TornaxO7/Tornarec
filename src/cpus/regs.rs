
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Regs {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,      R8Fiq,
    R9,      R9Fiq,
    R10,     R10Fiq,
    R11,     R11Fiq,
    R12,     R12Fiq,
    R13,     R13Fiq,  R13Svc,  R13Abt,  R13Irq,  R13Und, // SP
    R14,     R14Fiq,  R14Svc,  R14Abt,  R14Irq,  R14Und, // LR
    R15,     PC,
    CPSR,
             SPSRFiq, SPSRSvc, SPSRAbt, SPSRIrq, SPSRUnd,
}
