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
