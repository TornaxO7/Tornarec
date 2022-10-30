use std::{rc::Rc, cell::RefCell};

use nds_components::{
    screen::NDSScreens, cpus::architecture::Architecture,
};

pub mod jit;
pub mod nds_components;

pub type Addr = usize;
pub type Hertz = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NDSState {
}

#[derive(Debug)]
pub struct NDS {
    arm7_tdmi: Architecture,
    arm946_es: Architecture,
    screens: NDSScreens,
    state: Rc<RefCell<NDSState>>,
}
