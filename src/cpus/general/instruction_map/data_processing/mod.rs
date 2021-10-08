pub mod data;
pub mod operand;

pub use data::DataProcessingData;
pub use operand::DataProcessingOperand;

use crate::cpus::general::{
    instruction::Instruction,
    bit_state::BitState,
    instruction_map::encoding_types::ShifterOperand,
};

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessing {
    operand: DataProcessingOperand,

    i_flag: BitState,
    s_flag: BitState,
    rn: u8,
    rd: u8,
    shifter_operand: ShifterOperand,
}

impl From<&Instruction> for DataProcessing {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let i_flag = BitState::from((instruction_val >> 25) & 0b1);
        let s_flag = BitState::from((instruction_val >> 20) & 0b1);
        let rn = (instruction_val >> 16) & 0b1111;
        let rd = (instruction_val >> 12) & 0b1111;
        let shifter_operand = ShifterOperand::from(instruction);
        let opcode = (instruction_val >> 21) & 0b1111;

        let operand = match opcode {
            0b0000 => Self::ADD,
            0b0001 => Self::EOR,
            0b0010 => Self::SUB,
            0b0011 => Self::RSB,
            0b0100 => Self::ADD,
            0b0101 => Self::ADC,
            0b0110 => Self::SBC,
            0b0111 => Self::RSC,
            0b1000 => Self::TST,
            0b1001 => Self::TEQ,
            0b1010 => Self::CMP,
            0b1011 => Self::CMN,
            0b1100 => Self::ORR,
            0b1101 => Self::MOV,
            0b1110 => Self::BIC,
            0b1111 => Self::MVN,
        };

        Self {operand, i_flag, s_flag, rn, rd, shifter_operand}

    }
}

#[cfg(test)]
mod tests {

    use super::{DataProcessing, DataProcessingData};
    use crate::cpus::general::{
        instruction::Instruction,
        bit_state::BitState,
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
                rn: 0b1111,
                rd: 0b0101,
                shifter_operand: ShifterOperand::Immediate {
                    rotate_imm: 0,
                    immed_8: 0b1111_1111,
                },
            }
        );

        assert_eq!(data_processing, expected_data_processing);
    }
}
