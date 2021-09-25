use core::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataBlock(Vec<u8>);

impl DataBlock {
    pub fn get(&self) -> &Vec<u8> {
        &self.0
    }
}

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

#[cfg(test)]
mod tests {

    use super::DataBlock;

    #[test]
    fn get() {
        let data_block = DataBlock(vec![1]);
        
        assert_eq!(data_block.get(), &vec![1]);
    }

    #[test]
    fn from_slice() {
        let input: Vec<u8> = vec![1, 2, 3];

        let data_block = DataBlock::from(&input[..]);
        let expected_datablock = DataBlock(vec![1, 2, 3]);

        assert_eq!(data_block, expected_datablock);
    }

    #[test]
    fn from_vector() {
        let input: Vec<u8> = vec![1, 2, 3];

        let datablock = DataBlock::from(input);
        let expected_datablock = DataBlock(vec![1, 2, 3]);

        assert_eq!(datablock, expected_datablock);
    }
}
