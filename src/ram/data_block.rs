use core::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataBlock(Vec<u8>);

impl From<&[u8]> for DataBlock {
    fn from(data: &[u8]) -> Self {
        Self(data.to_vec())
    }
}

impl From<Vec<u8>> for DataBlock {
    fn from(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl Deref for DataBlock {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
