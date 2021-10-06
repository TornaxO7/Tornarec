pub mod error;

use error::AddressingMode1Error;

use crate::cpus::general::{
    instruction::Instruction,
    instruction_map::encoding_types::field::{
        Immed8,
        RotateImm,
    },
};

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode1 {
    Immediate {
        rotate_imm: u8,
        immed_8: u8,
    },
    ImmediateShifts {
        shift_imm: u8,
        shift: u8,
        rm: u8, 
    },
    RegisterShifts {
        rs: u8,
        shift: u8,
        rm: u8,
    },
}

impl AddressingMode1 {
    pub fn is_immediate(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();
        (instruction_val >> 25) & 0b111 == 0b001
    }
}

impl From<&Instruction> for AddressingMode1 {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();
        let encoding_type = (instruction_val >> 25) & 0b111;
        
        if encoding_type == 0b001 {

        } else if encoding_type == 0b000 (
            if (instruction_val >> 4) & 0b1 == 0b0 {

            } else if (instruction_val >> ) {
        } else {
            unreachable!("{}", AddressingMode1Error::UnknownAddressingMode(instruction.clone()));
        }
    }
}
