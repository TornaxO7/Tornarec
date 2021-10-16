#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum MoveImmediateToStatusRegisterError {
    #[error("[MoveImmediateToStatusRegisterError]: SBOConflict with instruction: {0:b}")]
    SBOConflict(u32),
}
