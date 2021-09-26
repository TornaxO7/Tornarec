use core::convert::{TryFrom, TryInto};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum WordError {
    #[error("Couldn't parse the given array into a wordbyte: {0:?}")]
    ParseError(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word(u32);

impl Word {
    pub fn get(&self) -> u32 {
        self.0
    }
}

impl TryFrom<&[u8]> for Word {
    type Error = WordError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        match slice.try_into() {
            Ok(array) => Ok(Self(u32::from_le_bytes(array))),
            Err(_) => Err(WordError::ParseError(slice.to_vec())),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{Word, WordError};
    use core::convert::TryFrom;

    #[test]
    fn get() {
        let number = 10;
        let word = Word(number);
        assert_eq!(word.get(), number);
    }

    #[test]
    fn try_from_slice() {
        let input: Vec<u8> = vec![0x4, 0x3, 0x2, 0x1];
        let wordbyte = Word::try_from(&input[..]).unwrap();

        assert_eq!(wordbyte.get().to_le_bytes(), [0x4, 0x3, 0x2, 0x1]);
    }

    #[test]
    fn fail_try_from_slice() {
        let input: Vec<u8> = vec![0x2, 0x1];
        let wordbyte = Word::try_from(&input[..]);

        assert_eq!(wordbyte, Err(WordError::ParseError(input)));
    }
}
