use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shift {
    Right,
    Left,
    ArithmeticShiftRight,
    RotateRight,
}

impl From<u32> for Shift {
    fn from(num: u32) -> Self {
        match num & 0b11 {
            0b00 => Self::Left,
            0b01 => Self::Right,
            0b10 => Self::ArithmeticShiftRight,
            0b11 => Self::RotateRight,
            _other => unreachable!("[Shift Error]: Unknown shift operation: {:b}", _other),
        }
    }
}
