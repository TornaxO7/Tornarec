pub mod cpus;
pub mod jit;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NDS {
    mem: Vec<u8>,
}
