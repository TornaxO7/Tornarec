use nds_components::screen::NDSScreens;

pub mod cpus;
pub mod jit;
pub mod nds_components;

pub type Addr = usize;

pub const WORKRAM_SIZE: usize = 32_000;

#[derive(Debug)]
pub struct NDS {
    mem: Vec<u8>,
    screens: NDSScreens,
}
