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
pub enum DataType {
    Byte(u8),
    Halfword(u16),
    Word(u32),
}

impl DataType {
    pub const BYTE_SIZE: u32 = 8;
    pub const HALFWORD_SIZE: u32 = 16;
    pub const WORD_SIZE: u32 = 32;

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
            Self::Byte(val) => u32::from(*val),
            Self::Halfword(val) => u32::from(*val),
            Self::Word(val) => *val,
        }
    }

    pub fn get_size(&self) -> u32 {
        match self {
            Self::Byte(_) => Self::BYTE_SIZE,
            Self::Halfword(_) => Self::HALFWORD_SIZE,
            Self::Word(_) => Self::WORD_SIZE,
        }
    }
}

impl Default for DataType {
    fn default() -> Self {
        Self::Word(0)
    }
}
