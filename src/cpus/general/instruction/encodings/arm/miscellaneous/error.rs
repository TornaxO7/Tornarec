#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum MiscellaneousError {
    #[error("[Miscellaneous Error]: SBOConflict with instruction: {0:b}")]
    SBOConflict(u32),

    #[error("[Miscellaneous Error]: SBZConflict with instruction: {0:b}")]
    SBZConflict(u32),
}
