use core::fmt;
use core::fmt::{UpperHex, LowerHex};
use core::ops::Add;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address(usize);

impl Address {
    pub fn get_ref(&self) -> &usize {
        &self.0
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

impl From<usize> for Address {
    fn from(num: usize) -> Self {
        Self(num)
    }
}

impl From<u32> for Address {
    fn from(num: u32) -> Self {
        Self(num as usize)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl UpperHex for Address {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:X}", self.0)
    }
}

impl LowerHex for Address {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:x}", self.0)
    }
}

impl Add<usize> for Address {
    type Output = Self;

    fn add(self, number: usize) -> Self {
        Self(self.0 + number)
    }
}
