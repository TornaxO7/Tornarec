use memmap2::Mmap;

use crate::Addr;

#[derive(Debug)]
pub struct CodeBlock {
    pub content: Mmap,
    pub start_addr: Addr,
}

impl CodeBlock {
}
