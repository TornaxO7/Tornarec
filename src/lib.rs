pub mod cpus;
pub mod jit;

pub type Addr = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NDS {
    mem: Vec<u8>,
}
