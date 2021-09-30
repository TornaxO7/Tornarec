pub mod operand;

pub use operand::DataProcessingOperand;

use crate::cpus::general::{
    instruction::Instruction,
    instruction_map::{
        InstructionMapTrait,
        encoding_types::{
            field::Opcode,
            ShifterOperand,
        },
    },
    register::types::RegisterIndex,
    bit_state::BitState,
};

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessing {
    pub i_flag: BitState,
    pub opcode: Opcode,
    pub s_flag: BitState,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub shifter_operand: ShifterOperand,
}

impl DataProcessing {
    pub fn get_operand(&self) -> DataProcessingOperand {
        match self.opcode.get_value_as_u8() {
            0b0001 => DataProcessingOperand::AND,
            0b0010 => DataProcessingOperand::EOR,
            0b0011 => DataProcessingOperand::SUB,
            0b0100 => DataProcessingOperand::RSB,
            0b0101 => DataProcessingOperand::ADD,
            0b0110 => DataProcessingOperand::ADC,
            0b0111 => DataProcessingOperand::SBC,
            0b1000 => DataProcessingOperand::RSC,
            0b1001 => DataProcessingOperand::TST,
            0b1010 => DataProcessingOperand::TEQ,
            0b1011 => DataProcessingOperand::CMP,
            0b1100 => DataProcessingOperand::CMN,
            0b1101 => DataProcessingOperand::ORR,
            0b1110 => DataProcessingOperand::MOV,
            0b1111 => DataProcessingOperand::BIC,
            _other => unreachable!("[DATA PROCESSING]: Reached unknown opcode: {:b}", _other),
        }
    }
}

impl From<&Instruction> for DataProcessing {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let i_flag = BitState::from((instruction_val >> 25) & 0b1);
        let opcode = Opcode::from((instruction_val >> 21) & 0b1111);
        let s_flag = BitState::from((instruction_val >> 20) & 0b1);
        let rn = RegisterIndex::from((instruction_val >> 16) & 0b1111);
        let rd = RegisterIndex::from((instruction_val >> 12) & 0b1111);
        let shifter_operand = ShifterOperand::from(instruction_val);

        Self {
            i_flag,
            opcode,
            s_flag,
            rn,
            rd,
            shifter_operand,
        }
    }
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
}
