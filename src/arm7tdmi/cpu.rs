//! The **main** file with the implementation of the cpu. All files in the directory are used
//! here.

use super::registers::{Distincts, SPSR, GeneralR8R12, Register};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("In THUMB state, only branch instructions can be made conditionally.")]
    InvalidStateForCondition,

    #[error("Unknown suffix: {0}")]
    UnknownSuffix(String),
    
    #[error("The System and User operating mode don't have a SPSR bank.")]
    NoSPSR,

    #[error("Regster {0} doesn't exist in Thumb state.")]
    ThumbStateUnavailableRegister(Register),

    #[error("Unknown operating mode code: {0:b}")]
    UnknownOperatingMode(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    ARM,
    THUMB,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatingMode {
    USR,
    FIQ,
    IRQ,
    SVC,
    ABT,
    SYS,
    UND
}

// == Structs ==
/// Represents the [ARM7TDMI] CPU.
///
/// [ARM7TDMI]: https://problemkaputt.de/gbatek.htm#armcpuoverview
pub struct Arm7TDMI {

    /// Registers from r0 to t7 (inclusive)
    reg: [u32; 8],

    /// Registers from r8 to r12 (inclusive). So index 0 refers to r8.
    reg8_12: [GeneralR8R12; 4],

    /// The program counter.
    pc: u32,

    /// The Stack pointer for each mode
    sp: Distincts,

    /// Used to store the value of pc when an exception is called. Each mode has its own
    /// register.
    lr: Distincts,

    /// **C**urrent **p**rogram **s**tatus **r**egister
    /// Bit 31     = N - Sign Flag
    /// Bit 30     = Z - Zero Flag
    /// Bit 29     = C - Carry Flag
    /// Bit 28     = V - Overflow Flag
    /// Bit 27 - 8 = Reserver (not used)
    /// Bit 7      = I  - IRQ disable
    /// Bit 6      = F  - FIQ disable
    /// Bit 5      = T  - State Bit
    /// Bit 4 - 0  = M4-M0 - Mode Bits
    cpsr: u32,

    /// Value in [`cpsr`] is moved to this register *of the respective exception-mode* if an
    /// exception arises.
    ///
    /// `SPSR` = **S**aved **P**rogram **S**tatus **R**egisters
    spsr: SPSR,
}

impl Arm7TDMI {
    
    /// Returns a new instance of the Cpu with the following values set:
    ///
    /// <details>
    ///
    /// ```text
    /// state:   State::ARM,
    /// reg:     vec![0; 8],
    /// reg8_12: vec![GeneralR8R12::default(); 4],
    /// pc:      0,
    /// sp:      Distincts::default(),
    /// lr:      Distincts::default(),
    /// cpsr:    0,
    /// spsr:    SPSR::default(),
    /// ```
    ///
    /// </details>
    ///
    pub fn new() -> Self {
        Self::default()
    }

    /// Decodes the given string and updates the values of the CPU.
    pub fn decode(&mut self, code: &str) {
    }

    /// Looks at bit 5 of the cpsr and returns the appropriate state of the CPU.
    pub fn get_state(&self) -> State {
        if self.cpsr & (1 << 5) > 0 {
            State::THUMB
        } else {
            State::ARM
        }
    }

    pub fn get_reg<'reg>(&'reg self, register: Register) -> Result<&'reg u32, Error> {
        let operating_mode = self.get_operating_mode()?;

        match register {
            Register::R0 | Register::R1 | Register::R2 | Register::R3 | Register::R4 | Register::R5 | Register::R6 | Register::R7 
                => Ok(&self.reg[register as usize]),
            Register::R8 => if operating_mode == OperatingMode::FIQ {
                Ok(&self.reg8_12[0].fiq)
            } else {
                Ok(&self.reg8_12[0].norm)
            },
            Register::R9 => if operating_mode == OperatingMode::FIQ {
                Ok(&self.reg8_12[1].fiq)
            } else {
                Ok(&self.reg8_12[1].norm)
            },
            Register::R10 => if operating_mode == OperatingMode::FIQ {
                Ok(&self.reg8_12[2].fiq)
            } else {
                Ok(&self.reg8_12[2].norm)
            },
            Register::R11 => if operating_mode == OperatingMode::FIQ {
                Ok(&self.reg8_12[3].fiq)
            } else {
                Ok(&self.reg8_12[3].norm)
            },
            Register::R12 => if operating_mode == OperatingMode::FIQ {
                Ok(&self.reg8_12[3].fiq)
            } else {
                Ok(&self.reg8_12[3].norm)
            },
            Register::SP => match operating_mode {
                OperatingMode::USR | OperatingMode::SYS => Ok(&self.sp.usr_sys),
                OperatingMode::FIQ => Ok(&self.sp.fiq),
                OperatingMode::IRQ => Ok(&self.sp.irq),
                OperatingMode::SVC => Ok(&self.sp.svc),
                OperatingMode::ABT => Ok(&self.sp.abt),
                OperatingMode::UND => Ok(&self.sp.und),
            },
            Register::LR => match operating_mode {
                OperatingMode::USR | OperatingMode::SYS => Ok(&self.lr.usr_sys),
                OperatingMode::FIQ => Ok(&self.lr.fiq),
                OperatingMode::IRQ => Ok(&self.lr.irq),
                OperatingMode::SVC => Ok(&self.lr.svc),
                OperatingMode::ABT => Ok(&self.lr.abt),
                OperatingMode::UND => Ok(&self.lr.und),
           },
           Register::CPSR => Ok(&self.cpsr),
           Register::SPSR => match operating_mode {
                OperatingMode::USR | OperatingMode::SYS => Err(Error::NoSPSR),
                OperatingMode::FIQ => Ok(&self.spsr.fiq),
                OperatingMode::IRQ => Ok(&self.spsr.irq),
                OperatingMode::SVC => Ok(&self.spsr.svc),
                OperatingMode::ABT => Ok(&self.spsr.abt),
                OperatingMode::UND => Ok(&self.spsr.und),
           },
           Register::PC => Ok(&self.pc)
        }
    }

    pub fn get_operating_mode(&self) -> Result<OperatingMode, Error> {
        let operating_mode = self.cpsr & 0b11111;
        match operating_mode {
            0b10000 => Ok(OperatingMode::USR),
            0b10001 => Ok(OperatingMode::FIQ),
            0b10010 => Ok(OperatingMode::IRQ),
            0b10011 => Ok(OperatingMode::SVC),
            0b10111 => Ok(OperatingMode::ABT),
            0b11011 => Ok(OperatingMode::UND),
            0b11111 => Ok(OperatingMode::SYS),
            _ => Err(Error::UnknownOperatingMode(operating_mode as usize))
        }
    }
}

// == Traits ==
impl Default for Arm7TDMI {
    fn default() -> Self {

        Self {
            reg:     [0; 8],
            reg8_12: [GeneralR8R12::default(); 4],
            pc:      0,
            sp:      Distincts::default(),
            lr:      Distincts::default(),
            cpsr:    0,
            spsr:    SPSR::default(),
        }
    }
}
