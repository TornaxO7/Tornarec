use std::ops::{
    Deref,
    Not,
};

use crate::ram::Word;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BitState(bool);

impl BitState {
    pub const SET: Self = BitState(true);
    pub const UNSET: Self = BitState(false);

    pub fn new(value: Word, index: u32) -> Self {
        let bit_value = ((value >> index) & 0b1) != 0;
        Self(bit_value)
    }
}

impl Deref for BitState {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Not for BitState {
    type Output = bool;

    fn not(self) -> Self::Output {
        !self.0
    }
}

impl From<BitState> for u32 {
    fn from(state: BitState) -> Self {
        match state {
            BitState::SET => 1,
            BitState::UNSET => 0,
        }
    }
}

impl Default for BitState {
    fn default() -> Self {
        Self::UNSET
    }
}
