pub mod cpus;
pub mod jit;
pub mod nds_components;

pub type Addr = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NDS {
    mem: Vec<u8>,
}
