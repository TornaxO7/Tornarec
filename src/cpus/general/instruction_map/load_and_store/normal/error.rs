#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum NormalError {
    #[error("[NORMAL LOAD AND STORE ERROR]: Unknown operand instruction: '{0:b}'")]
    UnknownOperand(u32),
}
