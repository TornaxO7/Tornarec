use core::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data(Vec<u8>);

impl From<Vec<u8>> for Data {
    fn from(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl Deref for Data {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
