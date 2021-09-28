pub mod prefetch;

pub use prefetch::Prefetch;

use crate::ram::data_types::{DataType, DataTypeSize};
use crate::ram::{Ram, Address};
use crate::cpus::instruction_map::InstructionMap;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum PipelineError {
    #[error("Invalid instruction size: {0:?}")]
    InvalidInstructionSize(DataTypeSize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pipeline {
    prefetch: Prefetch,
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
                    Ok(word) => self.prefetch = Prefetch::Success(word),
                    Err(err) => panic!("{}", err),
                },
                DataTypeSize::Halfword => match DataType::get_halfword(&ram[start_usize..end.get_as_usize()]) {
                    Ok(halfword) => self.prefetch = Prefetch::Success(halfword),
                    Err(err) => panic!("{}", err),
                },
                _ => unreachable!("{}", PipelineError::InvalidInstructionSize(instruction_size)),
            }
        }
    }

    pub fn decode(&mut self) {

    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            prefetch: Prefetch::from(DataType::default()),
            decoded_instruction: InstructionMap::Noop,
        }
    }
}
