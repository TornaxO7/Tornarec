use std::ops::Mul;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum DataTypeSize {
    Byte = 8,
    Halfword = 16,
    Word = 32,
}

impl Mul<u32> for DataTypeSize {
    type Output = u32;

    fn mul(self, num: u32) -> Self::Output {
        self as u32 * num
    }
}
