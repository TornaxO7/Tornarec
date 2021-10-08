use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
    instruction_map::encoding_types::ShifterOperand,
};

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingData {
    i_flag: BitState,
    s_flag: BitState,
    rn: u8,
    rd: u8,
    shifter_operand: ShifterOperand,
}

impl From<&Instruction> for DataProcessingData {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let i_flag = BitState::from((instruction_val >> 25) & 0b1);
        let s_flag = BitState::from((instruction_val >> 20) & 0b1);
        let rn = (instruction_val >> 16) & 0b1111;
        let rd = (instruction_val >> 12) & 0b1111;
        let shifter_operand = ShifterOperand::from(instruction);

        Self{i_flag, s_flag, rn, rd, shifter_operand}
    }
}
