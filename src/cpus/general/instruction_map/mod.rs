pub mod branch;
pub mod coprocessor;
pub mod data_processing;
pub mod exception_generating;
pub mod load_and_store;
pub mod status_register_transfer;

use crate::cpus::general::exception::Exception;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionMap {
    Exception(Exception),
    Noop,
}

impl Default for InstructionMap {
    fn default() -> Self {
        Self::Noop
    }
}
