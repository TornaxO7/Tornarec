#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ArmExecuterError {
    #[error("[Arm executer error]: Reached unpredictable caces.")]
    Unpredictable,
}
