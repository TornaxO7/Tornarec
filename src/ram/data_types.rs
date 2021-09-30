use core::convert::TryInto;
use core::ops::Add;

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
pub enum DataTypeSize {
    Byte = 8,
    Halfword = 16,
    Word = 32,
}

impl Add<DataTypeSize> for usize {
    type Output = usize;

    fn add(self, data_type_size: DataTypeSize) -> usize {
        match data_type_size {
            DataTypeSize::Byte     => self + DataTypeSize::Byte as usize,
            DataTypeSize::Halfword => self + DataTypeSize::Halfword as usize,
            DataTypeSize::Word     => self + DataTypeSize::Word as usize,
        }
    }
}

impl Add<DataTypeSize> for u32 {
    type Output = u32;

    fn add(self, data_type_size: DataTypeSize) -> u32 {
        match data_type_size {
            DataTypeSize::Byte => self + DataTypeSize::Byte as u32,
            DataTypeSize::Halfword => self + DataTypeSize::Halfword as u32,
            DataTypeSize::Word => self + DataTypeSize::Word as u32,
        }
    }
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
