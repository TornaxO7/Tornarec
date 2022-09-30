use nds_components::screen::NDSScreens;

pub mod cpus;
pub mod jit;
pub mod nds_components;

pub type Addr = usize;

#[derive(Debug)]
pub struct NDS {
    mem: Vec<u8>,
    screens: NDSScreens,
}
