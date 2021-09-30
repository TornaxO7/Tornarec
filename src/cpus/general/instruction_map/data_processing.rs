use crate::cpus::general::{
    instruction::Instruction,
    instruction_map::{
        InstructionMapTrait,
        encoding_types::DataProcessingEncoding,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataProcessing {
    AND(DataProcessingEncoding),
    EOR,
    SUB,
    RSB,
    ADD,
    ADC,
    SBC,
    RSC,
    TST,
    TEQ,
    CMP,
    CMN,
    ORR,
    MOV,
    BIC,
    MVN,
}

impl InstructionMapTrait for DataProcessing {
    fn is_matching(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();

        if (instruction_val >> 26) & 0b11 == 0b00 {
            true
        } else {
            false
        }
    }

    fn match_instruction(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();
        let opcode = (instruction_val >> 21) & 0b1111;

        match opcode {
            0b0001 => Self::AND(DataProcessingEncoding::from(instruction)),
            0b0010 => Self::EOR,
            0b0011 => Self::SUB,
            0b0100 => Self::RSB,
            0b0101 => Self::ADD,
            0b0110 => Self::ADC,
            0b0111 => Self::SBC,
            0b1000 => Self::RSC,
            0b1001 => Self::TST,
            0b1010 => Self::TEQ,
            0b1011 => Self::CMP,
            0b1100 => Self::CMN,
            0b1101 => Self::ORR,
            0b1110 => Self::MOV,
            0b1111 => Self::BIC,
            _other => unreachable!("[DATA PROCESSING]: Reached unknown opcode: {:b}", _other),
        }
    }
}
