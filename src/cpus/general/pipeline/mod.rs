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
    instruction::arm::ArmInstruction,
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
                OperatingState::Arm => get_arm_instruction(address, value, registers),
                OperatingState::Thumb => get_thumb_instruction(address, value, registers),
            },
            Prefetch::Invalid => panic!("Houston, we've a little problem..."),
        };

        self.decoded_instruction = Some(Box::new(decoded_instruction));
    }

    pub fn get_decoded_instruction(&self) -> Option<Box<dyn Instruction>> {
        self.decoded_instruction
    }
}

fn get_arm_instruction(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {}

fn get_thumb_instruction(
    address: &Address,
    value: &Word,
    registers: &Registers,
) -> ThumbInstruction {
}
