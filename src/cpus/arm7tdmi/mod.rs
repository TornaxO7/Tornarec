mod regs;

use std::time::Instant;

use super::Register;

pub const AMOUNT_REGS: usize = 16;
pub const AMOUNT_BANKED_REGS: usize = 5;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arm7TDMI {
    regs: [Register; AMOUNT_REGS],
    r13bank: [Register; AMOUNT_BANKED_REGS],
    r14bank: [Register; AMOUNT_BANKED_REGS],
    pc: Register,
    cpsr: Register,
    spsr: [Register; AMOUNT_BANKED_REGS - 1],
    tick: Instant,
}

impl Default for Arm7TDMI {
    fn default() -> Self {
        Self {
            regs: [Register::default(); AMOUNT_REGS],
            r13bank: [Register::default(); AMOUNT_BANKED_REGS],
            r14bank: [Register::default(); AMOUNT_BANKED_REGS],
            pc: Register::default(),
            cpsr: Register::default(),
            spsr: [Register::default(); AMOUNT_BANKED_REGS - 1],
            tick: Instant::now(),
        }
    }
}
