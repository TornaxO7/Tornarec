use crate::cpus::general::bit_state::BitState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchOperand {
    BOrBL {
        l: BitState,
        signed_immed_24: i32
    },
    Bx {
        rm: u8,
        switch_to_thumb: BitState,
    },
}
