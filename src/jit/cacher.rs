use fnv::FnvHashMap;
use crate::Addr;
use super::block::CodeBlock;

#[derive(Debug)]
pub struct Cacher {
    blocks: FnvHashMap<Addr, CodeBlock>,
}

impl Cacher {
    pub fn new() -> Self {
        Self {
            blocks: FnvHashMap::default(),
        }
    }
}
