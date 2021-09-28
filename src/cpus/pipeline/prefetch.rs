use crate::ram::data_types::DataType;

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prefetch {
    Success(DataType),
    Invalid,
}

impl From<DataType> for Prefetch {
    fn from(data_type: DataType) -> Self {
        Self::Success(data_type)
    }
}
