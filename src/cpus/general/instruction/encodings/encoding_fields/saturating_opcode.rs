use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaturatingOpcode {
    QADD,
    QSUB,
}

impl From<u32> for SaturatingOpcode {
    fn from(num: u32) -> Self {
        match num & 0b11 {
            0b00 => Self::QADD,
            0b01 => Self::QSUB,
            val => unreachable!("[SaturatingOpcode]: Unknown opcode: {:b}", val),
        }
    }
}
