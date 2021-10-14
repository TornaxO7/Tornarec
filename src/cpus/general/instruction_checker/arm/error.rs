#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ArmCheckerError {
    #[error("[Arm Checker Error]: Instruction '{0:b}' is unknown.")]
    UnknownInstruction(u32),
}
