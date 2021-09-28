use crate::ram::data_types::{DataType, DataTypeError};
use crate::cpus::instruction_map::InstructionMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pipeline {
    raw_instruction: Result<DataType, DataTypeError>,
    decoded_instruction: InstructionMap,
}

impl Pipeline {

    pub fn fetch(&mut self, raw_instruction: Result<DataType, DataTypeError>) {
        self.raw_instruction = raw_instruction;
    }

    pub fn decode(&mut self) {

    }

    pub fn execute(&self) {
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            raw_instruction: Ok(DataType::default()),
            decoded_instruction: InstructionMap::Noop,
        }
    }
}
