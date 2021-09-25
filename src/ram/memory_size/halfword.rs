use core::convert::{TryFrom, TryInto};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum HalfwordError {
    #[error("[HALFWORD-ERROR]: Couldn't parse the given array into a halfword: {0:?}")]
    ParseError(Vec<u8>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Halfword(u16);

impl Halfword {
    pub fn get(&self) -> u16 {
        self.0
    }
}

impl TryFrom<&[u8]> for Halfword {
    type Error = HalfwordError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        match slice.try_into() {
            Ok(array) => Ok(Self(u16::from_le_bytes(array))),
            Err(_) => Err(HalfwordError::ParseError(slice.to_vec())),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{Halfword, HalfwordError};
    use core::convert::TryFrom;

    #[test]
    fn test_get() {
        let number = 42;
        let halfword = Halfword(number);
        assert_eq!(halfword.get(), number);
    }

    #[test]
    fn test_try_from_slice() {
        let input: Vec<u8> = vec![0x4, 0x3];
        let halfword = Halfword::try_from(&input[..]).unwrap();

        assert_eq!(halfword.get().to_le_bytes(), [0x4, 0x3]);
    }

    #[test]
    fn test_fail_try_from_slice() {
        let input: Vec<u8> = vec![0x1];
        let halfword = Halfword::try_from(&input[..]);

        assert_eq!(halfword, Err(HalfwordError::ParseError(input)));
    }
}
