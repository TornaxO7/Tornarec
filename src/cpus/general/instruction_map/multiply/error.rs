#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum MultiplyError {
    #[error("[MULTIPLY ERROR]: Unknown multiply operand: {0:b}")]
    UnknownOperand(u32),

    #[error("[MULTIPLY ERROR]: MUL-Instruction doesn't have zero bits from 12:15: {0:b}!")]
    SBZConflict(u32),
}
