use core::convert::From;

use crate::cpus::general::{
    bit_state::BitState,
    register::types::RegisterIndex,
    instruction::Instruction,
    instruction_map::encoding_types::{ShifterOperand, Opcode},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingEncoding {
    pub i_flag: BitState,
    pub opcode: Opcode,
    pub s_flag: BitState,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub shifter_operand: ShifterOperand,
}

impl From<&Instruction> for DataProcessingEncoding {
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

