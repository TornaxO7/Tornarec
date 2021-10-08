#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum MiscellaneousError {
    #[error("[MISCELLANEOUS LOAD AND STORE ERROR]: SBZ fields aren't zero: '{0:b}'")]
    SBZConflict(u32),
    
    #[error("[MISCELLANEOUS LOAD AND STORE ERROR]: Unknown operand of instruction: '{0:b}'")]
    UnknownOperand(u32),
}
