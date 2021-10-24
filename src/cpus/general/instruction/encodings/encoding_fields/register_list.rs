use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterList(u16);

impl From<u32> for RegisterList {
    fn from(num: u32) -> Self {
        Self(u16::try_from(num & 0b1111_1111_1111_1111).unwrap())
    }
}
