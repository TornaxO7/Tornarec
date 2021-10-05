use core::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RegisterIndex(u8);

impl RegisterIndex {
    pub fn get_index(&self) -> u8 {
        self.0
    }
}

impl From<u32> for RegisterIndex {
    fn from(num: u32) -> Self {
        Self(u8::try_from(num & 0b1111).unwrap())
    }
}
