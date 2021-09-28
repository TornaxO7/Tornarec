use core::convert::TryInto;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum DataTypeError {
    #[error("[BYTE-ERROR]: Couldn't parse the given array into a byte: {0:?}")]
    ByteError(Vec<u8>),

    #[error("[HALFWORD-ERROR]: Couldn't parse the given array into a halfword: {0:?}")]
    HalfwordError(Vec<u8>),

    #[error("[WORD-ERROR]: Couldn't parse the given array into a wordbyte: {0:?}")]
    WordError(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataTypeSize;

impl DataTypeSize {
    pub const BYTE: u32     = 8;
    pub const HALFWORD: u32 = 16;
    pub const WORD: u32     = 32;
}

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
}

impl Default for DataType {
    fn default() -> Self {
        Self::Word(0)
    }
}
