//! The **main** file with the implementation of the cpu. All files in the directory are used
//! here.

use super::registers::{Distincts, SPSR, GeneralR8R12};

error_chain::error_chain! {
    errors {
        UnknownSuffix(suffix: String) {
            description("Unkown suffix."),
            display("{} is an unknown suffix.", suffix)
        }

        InvalidStateForCondition
    }
}

// == Enums ==
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    ARM,
    THUMB,
}

// == Structs ==
/// Represents the [ARM7TDMI] CPU.
///
/// [ARM7TDMI]: https://problemkaputt.de/gbatek.htm#armcpuoverview
pub struct Arm7TDMI {

    /// Registers from r0 to t7 (inclusive)
    pub reg: [u32; 8],

    /// Registers from r8 to r12 (inclusive). So index 0 refers to r8.
    pub reg8_12: [GeneralR8R12; 4],

    /// The program counter.
    pub pc: u32,

    /// The Stack pointer for each mode
    pub sp: Distincts,

    /// Used to store the value of pc when an exception is called. Each mode has its own
    /// register.
    pub lr: Distincts,

    /// **C**urrent **p**rogram **s**tatus **r**egister
    /// Bit 31     = N - Sign Flag
    /// Bit 30     = Z - Zero Flag
    /// Bit 29     = C - Carry Flag
    /// Bit 28     = V - Overflow Flag
    /// Bit 27     = Q - Sticky Overflow Flag
    /// Bit 26 - 8 = Reserver (not used)
    /// Bit 7      = I  - IRQ disable
    /// Bit 6      = F  - FIQ disable
    /// Bit 5      = T  - State Bit
    /// Bit 4 - 0  = M4-M0 - Mode Bits
    pub cpsr: u32,

    /// Value in [`cpsr`] is moved to this register *of the respective exception-mode* if an
    /// exception arises.
    ///
    /// `SPSR` = **S**aved **P**rogram **S**tatus **R**egisters
    pub spsr: SPSR,
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

    /// Looks at the last two characters of `mnemonic` and looks if the appropriate flag has been
    /// set correctly to return `true`. Otherwise, if the flag doesn't suit the condition it'll
    /// return `false`.
    ///
    /// # Example
    /// ```
    /// use tornarec::arm7tdmi::cpu::Arm7TDMI;
    ///
    /// fn main() {
    ///     let mut cpu = Arm7TDMI::new();
    ///
    ///     let suffix = String::from("EQ");
    ///     // our status register is empty, so this should return true
    ///     assert!(cpu.eval_suffix(&suffix).unwrap());
    ///
    ///     // let's set the z-flag to 1 (not equal)
    ///     cpu.cpsr = 1 << 30;
    ///     assert!(!cpu.eval_suffix(&suffix).unwrap());
    /// }
    /// ```
    pub fn eval_suffix(&self, mnemonic: &str) -> Result<bool> {
        let suffix_part = &mnemonic[mnemonic.len() - 2..mnemonic.len()];

        // get the status of the flags
        let v = (self.cpsr & (1 << 28)) > 0;
        let c = (self.cpsr & (1 << 29)) > 0;
        let z = (self.cpsr & (1 << 30)) == 0;
        let n = (self.cpsr & (1 << 31)) > 0;

        // In THUMB state, only branch instructions (jumps) can be made conditionally
        if self.get_state() == State::THUMB && mnemonic.chars().nth(0) != Some('J') {
            return Err(ErrorKind::InvalidStateForCondition.into());
        }

        let condition = match suffix_part {
            "EQ"        => z,
            "NE"        => !z,
            "CS" | "HS" => c,
            "CC" | "LO" => !c,
            "MI"        => n,
            "PL"        => !n,
            "VS"        => v,
            "VC"        => !v,
            "HI"        => c && !z,
            "LS"        => !c && z,
            "GE"        => n == v,
            "LT"        => n != v,
            "GT"        => z && n == v,
            "LE"        => z || n != v,
            "AL" | "NV" => true,
            _ => return Err(ErrorKind::UnknownSuffix(suffix_part.to_string()).into()),
        };

        Ok(condition)
    }
}

// == Traits ==
impl Default for Arm7TDMI {
    fn default() -> Self {

        Self {
            state:   State::ARM,
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
