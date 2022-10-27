use nds_components::{
    cpus::{arm7tdmi::Arm7TDMI, arm946es::Arm949Es},
    screen::NDSScreens,
};

pub mod jit;
pub mod nds_components;

pub type Addr = usize;

pub const WORKRAM_SIZE: usize = 32_000;

#[derive(Debug)]
pub struct NDS {
    screens: NDSScreens,
    arm7_tdmi: Arm7TDMI,
    arm946_es: Arm949Es,
}
