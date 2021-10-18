use crate::cpus::general::bit_state::BitState;

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionBits {
    pub n: BitState,
    pub z: BitState,
    pub c: BitState,
    pub v: BitState,
}

impl From<u32> for ConditionBits {
    fn from(num: u32) -> Self {
        let n = BitState::from(num >> 31);
        let z = BitState::from(num >> 30);
        let c = BitState::from(num >> 29);
        let v = BitState::from(num >> 28);

        Self { n, z, c, v }
    }
}
