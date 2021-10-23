use crate::cpus::general::OperatingMode;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum RegistersError {
    #[error("[Registers Error]: The operating mode '{0:?}' doesn't have a SPSR.")]
    NoSpsr(OperatingMode),
}
