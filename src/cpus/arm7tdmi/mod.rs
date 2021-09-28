pub mod condition_flag;
pub mod operating_mode;
pub mod operating_state;
pub mod interruption;
pub mod exception;
pub mod registers;

pub use registers::{FiqReg, Spsr, Cpsr, GeneralRegister};
pub use interruption::Interruption;
pub use operating_mode::OperatingMode;
pub use operating_state::OperatingState;
pub use condition_flag::ConditionFlag;
pub use exception::Exception;

use crate::cpus::state::State;
use crate::ram::Ram;
use crate::cpus::pipeline::Pipeline;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Arm7TDMI {
    r0_7:   [GeneralRegister; 8],
    r8_r12: [FiqReg; 4],
    // r13
    sp:     [GeneralRegister; OperatingMode::AMOUNT_MODES],
    // r14
    lr:     [GeneralRegister; OperatingMode::AMOUNT_MODES],
    // r15
    pc:     GeneralRegister,
    cpsr:   Cpsr,
    spsr:   Spsr,

    pipeline: Pipeline,
}

impl Arm7TDMI {

    pub fn reset(&self) -> Self {
        Self::default()
    }

    pub fn step(&mut self) {
        todo!();
    }

    pub fn fetch(&mut self, _ram: &Ram) {
        // let start = self.pc.get_as_usize();

        // TODO: 2.8.6
        // match self.cpsr.get_operating_state() {
        //     OperatingState::Arm => match DataType::get_word(&ram[start..start + DataTypeSize::WORD as usize]) {
        //         Ok(word) => self.pipeline.set_raw_instruction(word),
        //         Err(err) => panic!("{}", err),
        //     },
        //     OperatingState::Thumb => match DataType::get_halfword(&ram[start..start + DataTypeSize::HALFWORD as usize]) {
        //         Ok(halfword) => self.pipeline.set_raw_instruction(halfword),
        //         Err(err) => panic!("{}", err),
        //     },
        // };
    }

    pub fn decode(&self) {
        todo!();
    }

    pub fn execute(&mut self) {

        // Look if an FIQ or IRQ interrupt has been set, during execution
        match self.cpsr.get_operating_mode() {
            Ok(mode) => {
                if (mode == OperatingMode::Fiq)
                    && (self.cpsr.get_interrrupt_bit_state(Interruption::Fiq) != State::Unset) 
                {
                        self.exception(Exception::Fiq);
                }
                else if (mode == OperatingMode::Irq)
                    && (self.cpsr.get_interrrupt_bit_state(Interruption::Irq) != State::Unset)
                {
                    self.exception(Exception::Irq);
                }
            },
            Err(err) => panic!("{}", err),
        }
    }

    pub fn exception(&mut self, exception: Exception) {
        let in_arm_state = self.cpsr.get_operating_state() == OperatingState::Arm;

        match exception {
            Exception::Bl => {
                if in_arm_state {
                    self.lr[OperatingMode::as_usize(OperatingMode::Sys)] = self.pc.clone() + 2;
                } else {
                    self.lr[OperatingMode::as_usize(OperatingMode::Sys)] = self.pc.clone() + 4;
                }
                self.cpsr.set_operating_mode(OperatingMode::Sys);
                
            },
            Exception::Swi => {
                if in_arm_state {
                    self.lr[OperatingMode::as_usize(OperatingMode::Svc)] = self.pc.clone() + 2;
                } else {
                    self.lr[OperatingMode::as_usize(OperatingMode::Svc)] = self.pc.clone() + 4;
                }

                self.spsr.svc = self.cpsr.clone();
                self.cpsr.set_operating_mode(OperatingMode::Svc);
                
            },
            Exception::Udef => {
                if in_arm_state {
                    self.lr[OperatingMode::as_usize(OperatingMode::Und)] = self.pc.clone() + 2;
                } else {
                    self.lr[OperatingMode::as_usize(OperatingMode::Und)] = self.pc.clone() + 4;
                }

                self.spsr.und = self.cpsr.clone();
                self.cpsr.set_operating_mode(OperatingMode::Und);
            },
            Exception::Pabt => {
                self.lr[OperatingMode::as_usize(OperatingMode::Abt)] = self.pc.clone() + 4;
                self.spsr.abt = self.cpsr.clone();
                self.cpsr.set_operating_mode(OperatingMode::Abt);
            },
            Exception::Fiq  => {
                self.lr[OperatingMode::as_usize(OperatingMode::Fiq)] = self.pc.clone() + 4;
                self.spsr.fiq = self.cpsr.clone();
                self.cpsr.set_operating_mode(OperatingMode::Fiq);
                self.cpsr.set_interrupt_bit(Interruption::Irq, State::Set)
            },
            Exception::Irq  => {
                self.lr[OperatingMode::as_usize(OperatingMode::Irq)] = self.pc.clone() + 4;
                self.spsr.irq = self.cpsr.clone();
                self.cpsr.set_operating_mode(OperatingMode::Irq);
            },
            Exception::Dabt => {
                self.lr[OperatingMode::as_usize(OperatingMode::Abt)] = self.pc.clone() + 8;
                self.spsr.abt = self.cpsr.clone();
                self.cpsr.set_operating_mode(OperatingMode::Abt);
            },
            Exception::Reset => (),
        };

        self.cpsr.set_operating_state(OperatingState::Arm);
    }
}
