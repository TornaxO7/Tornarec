#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum DataTypeError {
    #[error("[BYTE-ERROR]: Couldn't parse the given array into a byte: {0:?}")]
    ByteError(Vec<u8>),

    #[error("[HALFWORD-ERROR]: Couldn't parse the given array into a halfword: {0:?}")]
    HalfwordError(Vec<u8>),

    #[error("[WORD-ERROR]: Couldn't parse the given array into a wordbyte: {0:?}")]
    WordError(Vec<u8>),
}
