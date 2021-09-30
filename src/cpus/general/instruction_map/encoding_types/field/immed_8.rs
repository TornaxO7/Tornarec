use core::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Immed8(u8);

impl Immed8 {
    pub fn get_value_as_u8(&self) -> u8 {
        self.0
    }
}

impl From<u32> for Immed8 {
    fn from(val: u32) -> Self {
        Self(u8::try_from(val & 0b1111_1111).unwrap())
    }
}
