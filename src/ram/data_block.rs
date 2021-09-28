use core::convert::TryFrom;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum DataBlockError {
    #[error("[DATABLOCK ERROR]: The datablock is too big: {0}")]
    OverflowBlock(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataBlock(Vec<u8>);

impl DataBlock {
    pub fn get_ref(&self) -> &Vec<u8> {
        &self.0
    }

    pub fn size(&self) -> u32 {
        match u32::try_from(self.0.len()) {
            Ok(num) => num,
            Err(_)  => {
                println!("{}", DataBlockError::OverflowBlock(self.0.len()));
                panic!();
            },
        }
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

#[cfg(test)]
mod tests {

    use super::DataBlock;

    #[test]
    fn get() {
        let data_block = DataBlock(vec![1]);
        
        assert_eq!(data_block.get_ref(), &vec![1]);
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
