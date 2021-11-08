use std::ops::{Mul, Sub};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataTypeSize {
    Byte,
    Halfword,
    Word,
    Custom(u32),
}

impl DataTypeSize {
    pub fn val(self) -> u32 {
        match self {
            Self::Byte => 8,
            Self::Halfword => 16,
            Self::Word => 32,
            Self::Custom(val) => val,
        }
    }
}

impl Mul<u32> for DataTypeSize {
    type Output = u32;

    fn mul(self, num: u32) -> Self::Output {
        let val = self.val();
        val * num
    }
}

impl Sub<DataTypeSize> for u32 {
    type Output = u32;

    fn sub(self, rhs: DataTypeSize) -> u32 {
        self - rhs.val()
    }
}
