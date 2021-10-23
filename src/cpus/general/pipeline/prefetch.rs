use crate::cpus::general::instruction::Instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prefetch {
    Success(Instruction),
    Invalid,
}

impl Default for Prefetch {
    fn default() -> Self {
        Self::Success(Instruction::default())
    }
}
