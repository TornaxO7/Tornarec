use core::convert::{TryFrom, TryInto};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ByteError {

    #[error("[BYTE-ERROR]: Couldn't parse the given array into a byte: {0:?}")]
    ParseError(Vec<u8>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Byte(u8);

impl TryFrom<&[u8]> for Byte {
    type Error = ByteError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        match slice.try_into() {
            Ok(array) => Ok(Self(u8::from_be_bytes(array))),
            Err(_) => Err(ByteError::ParseError(slice.to_vec())),
        }
    }
}
