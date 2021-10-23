use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shift {
    /// Arithmetic shift right
    ASR,
    /// Logical shift left
    LSL,
    /// Logical shift right
    LSR,
    /// (rotate right) or () rotate right with extend
    ROROrRRX,
}

impl From<u32> for Shift {
    fn from(num: u32) -> Self {
        match num & 0b11 {
            0b00 => Self::LSL,
            0b01 => Self::LSR,
            0b10 => Self::ASR,
            0b11 => Self::ROROrRRX,
            _other => unreachable!("[Shift Error]: Unknown shift operation: {:b}", _other),
        }
    }
}
