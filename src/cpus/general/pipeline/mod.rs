pub mod prefetch;

pub use prefetch::Prefetch;

use crate::{
    ram::{
        data_types::{DataType, DataTypeSize},
        Ram,
        Address,
    },
    cpus::general::{
        instruction::Instruction,
        register::Cpsr,
        OperatingState,
        InstructionMap,
    },
};

use core::convert::From;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum PipelineError {
    #[error("Invalid instruction size: {0:?}")]
    InvalidInstructionSize(DataTypeSize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pipeline {
    prefetch: Prefetch,
    // TODO: HERE
    decoded_instruction: InstructionMap,
}

impl Pipeline {

    pub fn fetch(&mut self, ram: &Ram, start: Address, instruction_size: DataTypeSize) {
        let end = Address::from(start.get_as_u32() + instruction_size.clone());
        let start_usize = start.get_as_usize();

        if end.get_as_u32() > ram.size() {
            self.prefetch = Prefetch::Invalid;
        } else {
            match instruction_size {
                DataTypeSize::Word => match DataType::get_word(&ram[start_usize..end.get_as_usize()]) {
                    Ok(word) => self.prefetch = Prefetch::Success(Instruction::from(word)),
                    Err(err) => panic!("{}", err),
                },
                DataTypeSize::Halfword => match DataType::get_halfword(&ram[start_usize..end.get_as_usize()]) {
                    Ok(halfword) => self.prefetch = Prefetch::Success(Instruction::from(halfword)),
                    Err(err) => panic!("{}", err),
                },
                _ => unreachable!("{}", PipelineError::InvalidInstructionSize(instruction_size)),
            }
        }
    }

    pub fn decode(&mut self, cpsr: &Cpsr) {
        match &self.prefetch {
            Prefetch::Success(instruction) => {
                if cpsr.get_operating_state() == OperatingState::Arm {
                    // get the condition
                    if cpsr.is_condition_set(instruction.get_condition_code_flag()) {
                        self.decoded_instruction = InstructionMap::get_arm_instruction(instruction);
                    } else {
                        self.decoded_instruction = InstructionMap::Noop;
                    }
                } else {
                    self.decoded_instruction = InstructionMap::get_thumb_instruction(instruction);
                }
            },
            Prefetch::Invalid => panic!("Houston, we've a little problem..."),
        }
    }

    pub fn get_decoded_instruction(&self) -> InstructionMap {
        self.decoded_instruction.clone()
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            prefetch: Prefetch::default(),
            decoded_instruction: InstructionMap::default(),
        }
    }
}
