use std::convert::From;

use super::instruction::arm::BitState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingState {
    Arm,
    Thumb,
}

impl From<BitState> for OperatingState {
    fn from(bitstate: BitState) -> Self {
        match bitstate {
            true => OperatingState::Thumb,
            false => OperatingState::Arm,
        }
    }
}
