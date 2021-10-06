pub mod error;
pub mod data;

pub use error::DataProcessingError;
pub use data::DataProcessingData;

use crate::cpus::general::instruction::Instruction;

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataProcessing {
    AND(DataProcessingData),
    EOR(DataProcessingData),
    SUB(DataProcessingData),
    RSB(DataProcessingData),
    ADD(DataProcessingData),
    ADC(DataProcessingData),
    SBC(DataProcessingData),
    RSC(DataProcessingData),
    TST(DataProcessingData),
    TEQ(DataProcessingData),
    CMP(DataProcessingData),
    CMN(DataProcessingData),
    ORR(DataProcessingData),
    MOV(DataProcessingData),
    BIC(DataProcessingData),
    MVN(DataProcessingData),
}

impl From<&Instruction> for DataProcessing {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();
        let opcode = (instruction_val >> 21) & 0b1111;
        let data = DataProcessingData::from(instruction);

        match opcode {
            0b0000 => Self::ADD(data),
            0b0001 => Self::EOR(data),
            0b0010 => Self::SUB(data),
            0b0011 => Self::RSB(data),
            0b0100 => Self::ADD(data),
            0b0101 => Self::ADC(data),
            0b0110 => Self::SBC(data),
            0b0111 => Self::RSC(data),
            0b1000 => Self::TST(data),
            0b1001 => Self::TEQ(data),
            0b1010 => Self::CMP(data),
            0b1011 => Self::CMN(data),
            0b1100 => Self::ORR(data),
            0b1101 => Self::MOV(data),
            0b1110 => Self::BIC(data),
            0b1111 => Self::MVN(data),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{DataProcessing, DataProcessingData};
    use crate::cpus::general::{
        instruction::Instruction,
        bit_state::BitState,
        instruction_map::encoding_types:: ShifterOperand,
        register::types::RegisterIndex,
    };
    use core::convert::From;

    #[test]
    fn test_from() {
        let instruction = Instruction::from(0b0000_00_1_0101_1_1111_0101_0000_1111_1111);
        let data_processing = DataProcessing::from(&instruction);

        let expected_data_processing = DataProcessing::ADC(
            DataProcessingData {
                i_flag: BitState::Set,
                s_flag: BitState::Set,
                rn: RegisterIndex::from(0b1111),
                rd: RegisterIndex::from(0b0101),
                shifter_operand: ShifterOperand::Immediate {
                    rotate_imm: RotateImm::from(0),
                    immed_8: Immed8::from(0b1111_1111),
                },
            }
        );

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
