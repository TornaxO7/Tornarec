#[derive(thiserror::Error, Clone, Debug, PartialEq, Eq)]
pub enum BranchError {
    #[error("[BRANCH ERROR]: Bit[8:19] should be ones! Instruction value: {0:b}")]
    SBOConflict(u32),
}
