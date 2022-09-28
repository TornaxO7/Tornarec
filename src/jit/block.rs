use crate::Addr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    content: Vec<u8>,
    start_addr: Addr,
}
