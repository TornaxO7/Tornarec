#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ArmExecuterError {
    #[error("[Arm executer error]: Reached unpredictable caces.")]
    Unpredictable,

    #[error("[Arm executer error]: Reached Jazelle extension instruction.")]
    NoJazelleSupport,

    #[error("[Arm executer error]: ARM5vTE instruction called fromi ARMv4T")]
    ARMv4TExecutesARM5vTE,
}
