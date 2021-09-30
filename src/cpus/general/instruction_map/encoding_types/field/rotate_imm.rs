use core::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RotateImm(u8);

impl RotateImm {
    pub fn get_value_as_u8(&self) -> u8 {
        self.0
    }
}

impl From<u32> for RotateImm {
    fn from(val: u32) -> Self {
        Self(u8::try_from(val & 0b1111_1111).unwrap())
    }
}
