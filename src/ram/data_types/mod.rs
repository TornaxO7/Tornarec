mod error;
mod size;

pub use error::DataTypeError;
pub use size::DataTypeSize;

use core::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    Byte(u8),
    Halfword(u16),
    Word(u32),
}

impl DataType {
    pub fn get_byte(slice: &[u8]) -> Result<Self, DataTypeError> {
        match slice.try_into() {
            Ok(array) => Ok(Self::Byte(u8::from_le_bytes(array))),
            Err(_) => Err(DataTypeError::ByteError(slice.to_vec())),
        }
    }

    pub fn get_halfword(slice: &[u8]) -> Result<Self, DataTypeError> {
        match slice.try_into() {
            Ok(array) => Ok(Self::Halfword(u16::from_le_bytes(array))),
            Err(_) => Err(DataTypeError::HalfwordError(slice.to_vec())),
        }
    }

    pub fn get_word(slice: &[u8]) -> Result<Self, DataTypeError> {
        match slice.try_into() {
            Ok(array) => Ok(Self::Word(u32::from_le_bytes(array))),
            Err(_) => Err(DataTypeError::WordError(slice.to_vec())),
        }
    }

    pub fn get_value_as_u32(&self) -> u32 {
        match self {
            Self::Byte(val) => u32::from(val.clone()),
            Self::Halfword(val) => u32::from(val.clone()),
            Self::Word(val) => u32::from(val.clone()),
        }
    }
}

impl Default for DataType {
    fn default() -> Self {
        Self::Word(0)
    }
}
