use fnv::FnvHashMap;
use crate::Addr;
use super::block::Block;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cacher {
    blocks: FnvHashMap<Addr, Block>,
}
