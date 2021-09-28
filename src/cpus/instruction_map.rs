#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionMap {
    Move(u32, u32),
    Noop,
}

impl Default for InstructionMap {
    fn default() -> Self {
        Self::Noop
    }
}
