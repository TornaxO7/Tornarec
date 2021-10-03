pub mod operand;
pub mod error;

pub use operand::DataProcessingOperand;
pub use error::DataProcessingError;

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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DataProcessing {
    pub i_flag: BitState,
    pub opcode: Opcode,
    pub s_flag: BitState,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub shifter_operand: ShifterOperand,
}

impl From<&Instruction> for DataProcessing {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let i_flag = BitState::from((instruction_val >> 25) & 0b1);
        let opcode = Opcode::from((instruction_val >> 21) & 0b1111);
        let s_flag = BitState::from((instruction_val >> 20) & 0b1);
        let rn = RegisterIndex::from((instruction_val >> 16) & 0b1111);
        let rd = RegisterIndex::from((instruction_val >> 12) & 0b1111);

        let shifter_operand: ShifterOperand;
        if i_flag.is_set() {
            shifter_operand = ShifterOperand::from_immediate(instruction);
        } else {
            shifter_operand = ShifterOperand::from_shifts(instruction);
        }

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

    type Operand = DataProcessingOperand;

    fn is_matching(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();

        if (instruction_val >> 26) & 0b11 == 0b00 {
            true
        } else {
            false
        }
    }

    fn get_operand(&self) -> Self::Operand {
        match self.opcode.get_value_as_u8() {
            0b0000 => DataProcessingOperand::AND,
            0b0001 => DataProcessingOperand::EOR,
            0b0010 => DataProcessingOperand::SUB,
            0b0011 => DataProcessingOperand::RSB,
            0b0100 => DataProcessingOperand::ADD,
            0b0101 => DataProcessingOperand::ADC,
            0b0110 => DataProcessingOperand::SBC,
            0b0111 => DataProcessingOperand::RSC,
            0b1000 => DataProcessingOperand::TST,
            0b1001 => DataProcessingOperand::TEQ,
            0b1010 => DataProcessingOperand::CMP,
            0b1011 => DataProcessingOperand::CMN,
            0b1100 => DataProcessingOperand::ORR,
            0b1101 => DataProcessingOperand::MOV,
            0b1110 => DataProcessingOperand::BIC,
            _other => unreachable!("{}", DataProcessingError::UnknownOpcode(u32::from(_other))),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::DataProcessing;
    use crate::cpus::general::{
        instruction::Instruction,
        bit_state::BitState,
        instruction_map::{
            encoding_types::{
                ShifterOperand,
                field::{
                    Opcode,
                    RotateImm,
                    Immed8,
                },
            },
            InstructionMapTrait,
        }, 
        register::types::RegisterIndex
    };
    use core::convert::From;

    #[test]
    fn test_from() {
        let instruction = Instruction::from(0b0000_00_1_1010_1_1111_0101_0000_1111_1111);
        let data_processing = DataProcessing::from(&instruction);

        let expected_data_processing = DataProcessing {
            i_flag: BitState::Set,
            opcode: Opcode::from(0b1010 as u32),
            s_flag: BitState::Set,
            rn: RegisterIndex::from(0b1111),
            rd: RegisterIndex::from(0b0101),
            shifter_operand: ShifterOperand::Immediate {
                rotate_imm: RotateImm::from(0),
                immed_8: Immed8::from(0b1111_1111),
            },
        };

        assert_eq!(data_processing, expected_data_processing);
    }

    #[test]
    fn is_matching() {
        let instruction = Instruction::from(0b1111_0011_1111_1111_1111_1111_1111_1111);
        let invalid_instruction = Instruction::from(0b1111_1111_1111_1111_1111_1111_1111_1111);

        assert!(DataProcessing::is_matching(&instruction));
        assert!(!DataProcessing::is_matching(&invalid_instruction));
    }
}
