#[derive(thiserror::Error, Clone, Debug, PartialEq, Eq)]
pub enum DataProcessingError {
    #[error("[DATA PROCESSING]: Reached unknown opcode: {0:b}")]
    UnknownOpcode(u32),
}
