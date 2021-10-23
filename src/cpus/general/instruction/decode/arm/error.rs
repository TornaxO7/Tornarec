#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ArmDecodeError {
    #[error("[Arm decode]: Undefined instruction: {0:b}")]
    UndefinedInstruction(u32),

    #[error("[Arm decode]: Reached unpredictable instruction: {0:b}")]
    UnpredictableInstruction(u32),
}
