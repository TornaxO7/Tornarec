pub mod registers;
pub mod condition_flag;
pub mod operating_mode;
pub mod operating_state;
pub mod interruption;

pub use registers::{FiqReg, Spsr};
pub use interruption::Interruption;
pub use operating_mode::OperatingMode;
pub use operating_state::OperatingState;
pub use condition_flag::ConditionFlag;

use crate::ram::{Ram, Address};
use crate::ram::data_types::{DataTypeSize, DataType};
use crate::cpus::state::State;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Arm7TDMIError {
    #[error("The following operating mode is unknown: {0:b}")]
    UnknownOperatingMode(u32),
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

    instruction: DataType,
}

impl Arm7TDMI {

    pub fn reset(&self) -> Self {
        Self::default()
    }

    pub fn step(&mut self) {
        todo!();
    }

    pub fn fetch(&mut self, ram: &Ram) {
        // Assume OperatingState::Arm first
        let start = self.pc.get();

        match self.get_operating_state() {
            OperatingState::Arm => match DataType::get_word(&ram[start..start + DataTypeSize::Word as usize]) {
                Ok(word) => self.instruction = word,
                Err(err) => panic!("{}", err),
            },
            OperatingState::Thumb => match DataType::get_halfword(&ram[start..start + DataTypeSize::Halfword as usize]) {
                Ok(halfword) => self.instruction = halfword,
                Err(err) => panic!("{}", err),
            },
        };
    }

    pub fn decode(&self) {
    }

    pub fn execute(&self) {
    }

    pub fn exception(&mut self) {
    }

    pub fn get_condition_state(&self, flag: ConditionFlag) -> State {
        match flag {
            ConditionFlag::N => State::from(self.cpsr >> 31),
            ConditionFlag::Z => State::from((self.cpsr >> 30) & 1),
            ConditionFlag::C => State::from((self.cpsr >> 29) & 1),
            ConditionFlag::V => State::from((self.cpsr >> 28) & 1),
        }
    }

    pub fn set_interrupt_bit(&mut self, interrupt: Interruption, state: State) {
        match interrupt {
            Interruption::Irq => match state {
                State::Unset => self.cpsr &= !(1 << 7),
                State::Set   => self.cpsr |= 1 << 7,
            },
            Interruption::Fiq => match state {
                State::Unset => self.cpsr &= !(1 << 6),
                State::Set   => self.cpsr |= 1 << 6,
            }
        }
    }

    pub fn get_operating_state(&self) -> OperatingState {
        if (self.cpsr >> 5) & 1 == 0 {
            OperatingState::Arm
        } else {
            OperatingState::Thumb
        }
    }

    pub fn get_mode(&self) -> Result<OperatingMode, Self> {
        match self.cpsr & 0b11111 {
            0b10000 => Ok(OperatingMode::Usr),
            0b10001 => Ok(OperatingMode::Fiq),
            0b10010 => Ok(OperatingMode::Irq),
            0b10011 => Ok(OperatingMode::Svc),
            0b10111 => Ok(OperatingMode::Abt),
            0b11011 => Ok(OperatingMode::Und),
            0b11111 => Ok(OperatingMode::Sys),
            _other => {
                println!("{}", Arm7TDMIError::UnknownOperatingMode(_other));
                return Err(self.reset());
            },
        }
    }
}
