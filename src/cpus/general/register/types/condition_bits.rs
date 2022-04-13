use core::convert::From;

use crate::cpus::general::instruction::arm::BitState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionBits {
    pub n: BitState,
    pub z: BitState,
    pub c: BitState,
    pub v: BitState,
    pub q: BitState,
}

impl From<u32> for ConditionBits {
    fn from(num: u32) -> Self {
        let n = BitState::from(((num >> 31) & 0b1) != 0);
        let z = BitState::from(((num >> 30) & 0b1) != 0);
        let c = BitState::from(((num >> 29) & 0b1) != 0);
        let v = BitState::from(((num >> 28) & 0b1) != 0);
        let q = BitState::from(((num >> 27) & 0b1) != 0);

        Self { n, z, c, v, q }
    }
}
