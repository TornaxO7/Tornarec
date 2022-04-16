use std::{convert::TryFrom, ops::Deref};

use crate::ram::Word;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Register(u8);

impl Register {
    pub fn new(encoding: Word, shift: u32, mask: u32) -> Self {
        let register = u8::try_from((encoding >> shift) & mask).unwrap();
        Self(register)
    }
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Deref for Register {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
