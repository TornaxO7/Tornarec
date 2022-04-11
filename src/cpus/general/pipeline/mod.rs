pub mod prefetch;

pub use prefetch::Prefetch;

use crate::{
    cpus::general::{
        instruction::{
            decode::DecodeData,
            Instruction,
        },
        register::Registers,
        InstructionMap,
        OperatingState,
    },
    ram::{
        data_types::DataType,
        Address,
        Ram,
    },
};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum PipelineError {
    #[error("Invalid instruction size: {0:?}")]
    InvalidInstructionSize(DataType),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Pipeline {
    prefetch: Prefetch,
    decoded_instruction: InstructionMap,
}

impl<'a> Pipeline {
    pub fn fetch(&mut self, ram: &Ram, start: Address, instruction_size: DataType) {
        let end: Address = start + instruction_size.get_size();

        if end > ram.size() {
            self.prefetch = Prefetch::Invalid;
        } else {
            match instruction_size {
                DataType::Word(_) => match DataType::get_word(&ram[start..end]) {
                    Ok(word) => {
                        self.prefetch = Prefetch::Success(Instruction {
                            address: start,
                            val: word.get_value_as_u32(),
                        })
                    }
                    Err(err) => panic!("{}", err),
                },
                DataType::Halfword(_) => match DataType::get_halfword(&ram[start..end]) {
                    Ok(halfword) => {
                        self.prefetch = Prefetch::Success(Instruction {
                            address: start,
                            val: halfword.get_value_as_u32(),
                        })
                    }
                    Err(err) => panic!("{}", err),
                },
                _ => unreachable!(
                    "{}",
                    PipelineError::InvalidInstructionSize(instruction_size)
                ),
            }
        }
    }

    pub fn decode(&mut self, registers: &'a Registers) {
        let cpsr = registers.get_ref_cpsr();

        let decoded_instruction = match &self.prefetch {
            Prefetch::Success(instruction) => {
                let data = DecodeData::new(instruction.clone(), registers);

                if cpsr.get_operating_state() == OperatingState::Arm {
                    if cpsr.is_condition_set(data.instruction.get_condition_code_flag()) {
                        InstructionMap::get_arm_instruction(data)
                    } else {
                        InstructionMap::Noop
                    }
                } else {
                    InstructionMap::get_thumb_instruction(data)
                }
            }
            Prefetch::Invalid => panic!("Houston, we've a little problem..."),
        };

        self.decoded_instruction = decoded_instruction;
    }

    pub fn get_decoded_instruction(&self) -> InstructionMap {
        self.decoded_instruction.clone()
    }
}
