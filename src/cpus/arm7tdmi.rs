pub mod regs;

use super::Register;

pub const AMOUNT_REGS: usize = 16;
pub const AMOUNT_BANKED_REGS: usize = 5;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arm7TDMI {
    regs: [Register; AMOUNT_REGS],
    cpsr: Register,
    r14bank: [Register; AMOUNT_BANKED_REGS],
}

impl Arm7TDMI {
}
