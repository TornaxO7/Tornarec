pub mod arm7tdmi;
pub mod arm946es;
pub mod architecture;

pub type Register = u64;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Endianes {
    Little,
    Big,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperatingMode {
    /// User mode
    Usr,

    /// Fast Interrupt
    Fiq,

    /// Interrupt
    Irq,

    /// Supervisor
    Svc,

    /// Abort
    Abt,

    /// System
    Sys,

    /// Undefined
    Und,
}

impl OperatingMode {
    pub const AMOUNT_MODES: usize = 7;
}
