use crate::cpus::general::BitState;

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldMask {
    m1: BitState,
    m2: BitState,
    m3: BitState,
    m4: BitState,
}

impl From<u8> for FieldMask {
    fn from(num: u8) -> Self {
        Self {
            m1: BitState::from(num & 0b1),
            m2: BitState::from((num >> 1) & 0b1),
            m3: BitState::from((num >> 2) & 0b1),
            m4: BitState::from((num >> 3) & 0b1),
        }
    }
}

impl From<u32> for FieldMask {
    fn from(num: u32) -> Self {
        Self {
            m1: BitState::from(num & 0b1),
            m2: BitState::from((num >> 1) & 0b1),
            m3: BitState::from((num >> 2) & 0b1),
            m4: BitState::from((num >> 3) & 0b1),
        }
    }
}
