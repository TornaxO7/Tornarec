use core::convert::{TryFrom, TryInto};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum HalfwordError {
    #[error("[HALFWORD-ERROR]: Couldn't parse the given array into a halfword: {0:?}")]
    ParseError(Vec<u8>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Halfword(u16);

impl TryFrom<&[u8]> for Halfword {
    type Error = HalfwordError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        match slice.try_into() {
            Ok(array) => Ok(Self(u16::from_be_bytes(array))),
            Err(_) => Err(HalfwordError::ParseError(slice.to_vec())),
        }
    }
}
