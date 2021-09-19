//! Some helper structs to represent the registers like for the different modes.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum Register {
    R0, R1, R2, R3, R4, R5, R6, R7,
    R8, R9, R10, R11, R12,
    SP, LR,
    PC,
    CPSR,
    SPSR,
}

impl fmt::Display for Register {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::R0 => write!(formatter, "r0"),
            Self::R1 => write!(formatter, "r1"),
            Self::R2 => write!(formatter, "r2"),
            Self::R3 => write!(formatter, "r3"),
            Self::R4 => write!(formatter, "r4"),
            Self::R5 => write!(formatter, "r5"),
            Self::R6 => write!(formatter, "r6"),
            Self::R7 => write!(formatter, "r7"),
            Self::R8 => write!(formatter, "r8"),
            Self::R9 => write!(formatter, "r9"),
            Self::R10 => write!(formatter, "r10"),
            Self::R11 => write!(formatter, "r11"),
            Self::R12 => write!(formatter, "r12"),
            Self::SP => write!(formatter, "sp"),
            Self::LR => write!(formatter, "lr"),
            Self::PC => write!(formatter, "pc"),
            Self::CPSR => write!(formatter, "cpsr"),
            Self::SPSR => write!(formatter, "spsr"),
        }
    }
}

// == General r8 - r12 ==
/// Represents one register from r8 to r12, since the FIQ mode has its own registers in this range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneralR8R12 {
    pub norm: u32,
    pub fiq:  u32,
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
    /// User and System mode
    pub usr_sys: u32,
    /// FIQ mode
    pub fiq: u32,
    /// Supervisor mode
    pub svc: u32,
    /// Abort mode
    pub abt: u32,
    /// IRQ mode
    pub irq: u32,
    /// Undefined mode
    pub und: u32,
}

// -- Traits --
impl Default for Distincts {
    fn default() -> Self {
        Self {
            usr_sys: 0,
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
    pub fiq: u32,
    /// Supervisor mode
    pub svc: u32,
    /// Abort mode
    pub abt: u32,
    /// IRQ mode
    pub irq: u32,
    /// Undefined mode
    pub und: u32,
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
