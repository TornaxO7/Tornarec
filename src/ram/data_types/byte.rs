use core::convert::{TryFrom, TryInto};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ByteError {

    #[error("[BYTE-ERROR]: Couldn't parse the given array into a byte: {0:?}")]
    ParseError(Vec<u8>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Byte(u8);

impl Byte {
    pub fn get(&self) -> u8 {
        self.0
    }
}

impl TryFrom<&[u8]> for Byte {
    type Error = ByteError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        match slice.try_into() {
            Ok(array) => Ok(Self(u8::from_le_bytes(array))),
            Err(_) => Err(ByteError::ParseError(slice.to_vec())),
        }
    }
}

#[cfg(test)]
mod tests {
    
    use super::{Byte, ByteError};
    use core::convert::TryFrom;

    #[test]
    fn get() {
        let number = 10;
        let byte = Byte(number);
        assert_eq!(byte.get(), number);
    }

    #[test]
    fn try_from_slice() {
        let input: Vec<u8> = vec![0x4];
        let byte = Byte::try_from(&input[..]).unwrap();

        assert_eq!(byte.get().to_le_bytes(), [0x4]);
    }

    #[test]
    fn fail_try_from_slice() {
        let input: Vec<u8> = vec![0x4, 0x3, 0x2, 0x1];
        let byte = Byte::try_from(&input[..]);

        assert_eq!(byte, Err(ByteError::ParseError(input)));
    }
}
