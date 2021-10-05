#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadAndStoreOperand {
    LDR,
    LDRB,
    LDRBT,
    LDRH,
    LDRSB,
    LDRSH,
    LDRT,
    STR,
    STRB,
    STRBT,
    STRH,
    STRT,
}
