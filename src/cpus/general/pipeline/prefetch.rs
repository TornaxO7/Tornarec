use crate::ram::{
    data_types::DataType,
    Address, Word,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prefetch {
    Success { address: Address, value: Word },
    Invalid,
}

impl Default for Prefetch {
    fn default() -> Self {
        Self::Invalid
    }
}
