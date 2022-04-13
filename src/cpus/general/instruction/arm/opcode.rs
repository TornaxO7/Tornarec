#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmOpcode {
    ADC,
    ADD,
    AND,
    B,
    BL,
    BIC,
    BKPT,
    BLX1,
    BLX2,
    BX,
    BXJ,
    CDP,
    CLZ,
    CMN,
    CMP,
    CPS,
    CPY,
    EOR,
    LDC,
    LDM1,
    LDM2,
    LDM3,
    LDR,
    LDRB,
    LDRBT,
    LDRD,
    LDREX,
    LDRH,
    LDRSB,
    LDRSH,
    LDRT,
    MCR,
    MCRR,
    MLA,
    MOV,
    MRC,
    MRRC,
    MRS,
    MSR,
    MUL,
    MVN,
    ORR,
    PKHBT,
    PKHTB,
    PLD,
    QADD,
    QDADD,
    QDSUB,
    QSUB,
    QSUB106,
    REV,
    REV16,
    REVSH,
    RFE,
    RSB,
    RSC,
    SBC,
    SEL,
    SMLAXY,
    SMLALXY,
    SMLALD,
    SMLAWY,
    SMLSD,
    SMLSLD,
    SMMLA,
    SMMLS,
    SMMUL,
    SMUAD,
    SMULXY,
    SMULL,
    SMULWY,
    SMUSD,
    SRS,
    SSAT,
    SSAT16,
    STC,
    STM1,
    SMT2,
    STR,
    STRB,
    STRBT,
    STRD,
    STREX,
    STRH,
    STRT,
    SUB,
    SWI,
    SWP,
    SWPB,
    TEQ,
    TST,
    UHSUB16,
    UMAAL,
    UMLAL,
    UMULL,
    USADA8,
    USAT,
    USAT16,
    NOOP,
}
