use crate::cpus::endianess::Endianess;
use crate::ram::{Ram, Address};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatingState {Arm, Thumb}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum OperatingMode {Usr, Fiq, Irq, Svc, Abt, Sys, Und, AmountModes}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FiqReg {reg: u32, fiq: u32}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Spsr {
    fiq: u32,
    svc: u32,
    abt: u32,
    irq: u32,
    und: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Arm7TDMI {
    r0_7:   [u32; 8],
    r8_r12: [FiqReg; 4],
    sp:     [Address; OperatingMode::AmountModes as usize],
    lr:     [Address; OperatingMode::AmountModes as usize],
    pc:     Address,
    cpsr:   u32,
    spsr:   Spsr,
}

impl Arm7TDMI {

    pub fn step(&mut self) {
    }

    pub fn fetch(&mut self, ram: &Ram) {

    }

    pub fn decode(&self) {
    }

    pub fn execute(&self) {
    }

    pub fn get_endianess(&self) -> Endianess {
        Endianess::Big
    }
}
