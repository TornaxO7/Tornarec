#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum CpsrAccessError {
    #[error("[CPSR ACCESS ERROR]: Unknown operand code: {0:b}.")]
    UnknownOperand(u32),

    #[error("[CPSR ACCESS ERROR]: The bits at {0}:{1} aren't set to one in the following instruction: {2:b}")]
    SBOConflict(u32, u32, u32),

    #[error("[CPSR ACCESS ERROR]: The bits at {0}:{1} aren't set to zero in the following instruction: {2:b}")]
    SBZConflict(u32, u32, u32),
}
