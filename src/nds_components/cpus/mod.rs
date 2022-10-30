pub mod architecture;
pub mod regs;

pub type Register = u64;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Endianes {
    Little,
    Big,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperatingState {
    Arm,
    Thumb,
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
