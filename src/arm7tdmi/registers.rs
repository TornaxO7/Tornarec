//! Some helper structs to represent the registers like for the different modes.

// == General r8 - r12 ==
/// Represents one register from r8 to r12, since the FIQ mode has its own registers in this range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneralR8R12 {
    norm: u32,
    fiq:  u32,
}

// -- Traits --
impl Default for GeneralR8R12 {
    fn default() -> Self {
        Self {
            norm: 0,
            fiq:  0,
        }
    }
}

// == Distincts ==
/// Each mode gets their own registers. This struct is mainly used as the stack register and
/// LR register for each mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Distincts {
    /// User mode
    user: u32,
    /// FIQ mode
    fiq: u32,
    /// Supervisor mode
    svc: u32,
    /// Abort mode
    abt: u32,
    /// IRQ mode
    irq: u32,
    /// Undefined mode
    und: u32,
}

// -- Traits --
impl Default for Distincts {
    fn default() -> Self {
        Self {
            user: 0,
            fiq:  0,
            svc:  0,
            abt:  0,
            irq:  0,
            und:  0,
        }
    }
}

// == SPSR ==
/// Represents the SPSR register for the appropriate mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SPSR {
    /// FIQ mode
    fiq: u32,
    /// Supervisor mode
    svc: u32,
    /// Abort mode
    abt: u32,
    /// IRQ mode
    irq: u32,
    /// Undefined mode
    und: u32,
}

impl Default for SPSR {
    fn default() -> Self {
        Self {
            fiq: 0,
            svc: 0,
            abt: 0,
            irq: 0,
            und: 0,
        }
    }
}
