mod decode;
pub mod prefetch;

pub use prefetch::Prefetch;

use crate::{
    cpus::general::{
        register::Registers,
        OperatingState,
    },
    ram::{
        data_types::DataType,
        Address,
        Ram,
        Word,
    },
};

use super::{
    instruction::{
        arm::get_arm_instruction,
        thumb::get_thumb_instruction,
    },
    Instruction,
};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum PipelineError {
    #[error("Invalid instruction size: {0:?}")]
    InvalidInstructionSize(DataType),
}

#[derive(Debug, Default)]
pub struct Pipeline {
    prefetch: Prefetch,
    decoded_instruction: Option<Box<dyn Instruction>>,
}

impl<'a> Pipeline {
    pub fn fetch(&mut self, ram: &Ram, start: Address, instruction_size: DataType) {
        let end: Address = start + instruction_size.get_size();

        if end > ram.size() {
            self.prefetch = Prefetch::Invalid;
            return;
        }

        match instruction_size {
            DataType::Word => {
                self.prefetch = Prefetch::Success {
                    address: start,
                    value: DataType::get_word(&ram[start..end]).unwrap(),
                }
            }
            DataType::Halfword => {
                self.prefetch = Prefetch::Success {
                    address: start,
                    value: Word::from(DataType::get_halfword(&ram[start..end]).unwrap()),
                }
            }
            _ => unreachable!(
                "{}",
                PipelineError::InvalidInstructionSize(instruction_size)
            ),
        }
    }

    pub fn decode(&mut self, registers: &'a Registers) {
        let cpsr = registers.get_ref_cpsr();

        let decoded_instruction = match &self.prefetch {
            Prefetch::Success { address, value } => match cpsr.get_operating_state() {
                OperatingState::Arm => {
                    Box::new(get_arm_instruction(address, value, registers)) as Box<dyn Instruction>
                }
                OperatingState::Thumb => Box::new(get_thumb_instruction(address, value, registers))
                    as Box<dyn Instruction>,
            },
            Prefetch::Invalid => panic!("Houston, we've a little problem..."),
        };

        self.decoded_instruction = Some(decoded_instruction);
    }

    pub fn get_decoded_instruction(&self) -> Option<Box<dyn Instruction>> {
        self.decoded_instruction
    }
}
