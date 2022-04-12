use core::convert::TryInto;

use super::{Byte, Halfword, Word};

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
pub enum DataType {
    Byte,
    Halfword,
    Word,
}

impl DataType {
    pub const BYTE_SIZE: u32 = 8;
    pub const HALFWORD_SIZE: u32 = 16;
    pub const WORD_SIZE: u32 = 32;

    pub fn get_byte(slice: &[u8]) -> Result<Byte, DataTypeError> {
        match slice.try_into() {
            Ok(array) => Ok(u8::from_le_bytes(array)),
            Err(_) => Err(DataTypeError::ByteError(slice.to_vec())),
        }
    }

    pub fn get_halfword(slice: &[u8]) -> Result<Halfword, DataTypeError> {
        match slice.try_into() {
            Ok(array) => Ok(u16::from_le_bytes(array)),
            Err(_) => Err(DataTypeError::HalfwordError(slice.to_vec())),
        }
    }

    pub fn get_word(slice: &[u8]) -> Result<Word, DataTypeError> {
        match slice.try_into() {
            Ok(array) => Ok(u32::from_le_bytes(array)),
            Err(_) => Err(DataTypeError::WordError(slice.to_vec())),
        }
    }
}
