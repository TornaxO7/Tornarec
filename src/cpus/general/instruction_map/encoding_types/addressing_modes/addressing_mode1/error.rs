use crate::cpus::general::instruction::Instruction;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode1Error {
    #[error("[ADDRESSING MODE 1 ERROR]: Unknown instruction encoding: {0:#?}")]
    UnknownAddressingMode(Instruction),

    #[error("[ADDRESSING MODE 1 ERROR]: Unknown shift mode: {0:#?}")]
    UnknownShift(Instruction),
}
