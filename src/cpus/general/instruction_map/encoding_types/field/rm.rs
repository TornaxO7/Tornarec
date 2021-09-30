use core::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rm(u8);

impl From<u32> for Rm {
    fn from(val: u32) -> Self {
        Self(u8::try_from(val & 0b1111_1111).unwrap())
    }
}
