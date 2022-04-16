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
        let n = BitState::new(num, 31);
        let z = BitState::new(num, 30);
        let c = BitState::new(num, 29);
        let v = BitState::new(num, 28);
        let q = BitState::new(num, 27);

        Self { n, z, c, v, q }
    }
}
