use core::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Opcode(u8);

impl Opcode {
    pub fn get_value_as_u8(&self) -> u8 {
        self.0
    }
}

impl From<u32> for Opcode {
    fn from(val: u32) -> Self {
        Self(u8::try_from(val & 0b1111_1111).unwrap())
    }
}

impl From<u8> for Opcode {
    fn from(val: u8) -> Self {
        Self(val)
    }
}
