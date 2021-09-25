use core::convert::{TryFrom, TryInto};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum WordError {
    #[error("Couldn't parse the given array into a wordbyte: {0:?}")]
    ParseError(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word(u32);

impl TryFrom<&[u8]> for Word {
    type Error = WordError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        match slice.try_into() {
            Ok(array) => Ok(Self(u32::from_be_bytes(array))),
            Err(_) => Err(WordError::ParseError(slice.to_vec())),
        }
    }
}
