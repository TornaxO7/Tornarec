pub mod normal;
pub mod miscellaneous;

use normal::Normal;
use miscellaneous::Miscellaneous;
use crate::cpus::general::instruction::Instruction;

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadAndStore {
    Normal(Normal),
    Miscellaneous(Miscellaneous),
}

impl From<&Instruction> for LoadAndStore {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        if (instruction_val >> 26) & 0b11 == 0b01 {
            Self::Normal(Normal::from(instruction))
        } else {
            Self::Miscellaneous(Miscellaneous::from(instruction))
        }
    }
}
