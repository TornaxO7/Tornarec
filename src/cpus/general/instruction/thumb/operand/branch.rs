use crate::cpus::general::instruction::{
    thumb::types::Register,
    types::BitState,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Branch {
    Conditional { cond: u8, offset: u8 },
    Unconditional { h: BitState, offset: u16 },
    Exchange { h: BitState, rm: Register },
}
