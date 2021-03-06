use std::fmt;

use crate::ram::Word;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum OperatingMode {
    Usr,
    Fiq,
    Irq,
    Svc,
    Abt,
    Sys,
    Und,
}

impl OperatingMode {
    pub const AMOUNT_MODES: usize = 7;

    pub const fn as_u32(operating_mode: OperatingMode) -> u32 {
        match operating_mode {
            OperatingMode::Usr | OperatingMode::Sys => 0,
            OperatingMode::Fiq => 1,
            OperatingMode::Irq => 2,
            OperatingMode::Svc => 3,
            OperatingMode::Abt => 4,
            OperatingMode::Und => 5,
        }
    }

    pub const fn as_usize(operating_mode: OperatingMode) -> usize {
        match operating_mode {
            OperatingMode::Usr | OperatingMode::Sys => 0,
            OperatingMode::Fiq => 1,
            OperatingMode::Irq => 2,
            OperatingMode::Svc => 3,
            OperatingMode::Abt => 4,
            OperatingMode::Und => 5,
        }
    }
}

impl fmt::Display for OperatingMode {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut name = String::new();

        match self {
            OperatingMode::Usr => name.push_str("User"),
            OperatingMode::Fiq => name.push_str("Fiq"),
            OperatingMode::Irq => name.push_str("Irq"),
            OperatingMode::Svc => name.push_str("Supervisor"),
            OperatingMode::Abt => name.push_str("Abort"),
            OperatingMode::Sys => name.push_str("System"),
            OperatingMode::Und => name.push_str("Undefined"),
        }

        write!(fmt, "{}", name)
    }
}

impl From<Word> for OperatingMode {
    fn from(word: Word) -> Self {
        let bit4_0 = word & 0b1111;

        match bit4_0 {
            0b0000 => Self::Usr,
            0b0001 => Self::Fiq,
            0b0010 => Self::Irq,
            0b0011 => Self::Svc,
            0b0111 => Self::Abt,
            0b1011 => Self::Und,
            0b1111 => Self::Sys,
            _ => unreachable!("Unknown operating mode bits: {}", bit4_0),
        }
    }
}
