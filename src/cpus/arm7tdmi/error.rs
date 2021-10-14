#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Arm7TDMIError {
    #[error("[Arm7TDMI Error]: This should never happen... double unknown operating mode.")]
    UnreachableOperatingMode,
}
