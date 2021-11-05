use std::convert::From;

use super::BitState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingState {
    Arm,
    Thumb,
}

impl From<BitState> for OperatingState {
    fn from(bitstate: BitState) -> Self {
        match bitstate {
            BitState::Set => OperatingState::Thumb,
            BitState::Unset => OperatingState::Arm,
        }
    }
}
