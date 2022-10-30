use std::time::Instant;

use super::{Register, Endianes, OperatingState, OperatingMode};

pub const AMOUNT_REGS: usize = 16;
pub const AMOUNT_BANKED_REGS: usize = 5;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Architecture {
    pub regs: [Register; AMOUNT_REGS],
    pub r13bank: [Register; AMOUNT_BANKED_REGS],
    pub r14bank: [Register; AMOUNT_BANKED_REGS],
    pub pc: Register,
    pub cpsr: Register,
    pub spsr: [Register; AMOUNT_BANKED_REGS - 1],
    pub tick: Instant,
    pub endianes: Endianes,
    pub op_state: OperatingState,
    pub op_mode: OperatingMode,
}

impl Default for Architecture {
    fn default() -> Self {
        Self {
            regs: [Register::default(); AMOUNT_REGS],
            r13bank: [Register::default(); AMOUNT_BANKED_REGS],
            r14bank: [Register::default(); AMOUNT_BANKED_REGS],
            pc: Register::default(),
            cpsr: Register::default(),
            spsr: [Register::default(); AMOUNT_BANKED_REGS - 1],
            tick: Instant::now(),
            endianes: Endianes::Little,
            op_state: OperatingState::Arm,
            op_mode: OperatingMode::Sys,
        }
    }
}

impl Architecture {
    pub fn new() -> Self {
        Self::default()
    }
}