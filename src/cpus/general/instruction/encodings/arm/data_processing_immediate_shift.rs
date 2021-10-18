use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
    register::RegisterName,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingImmediateShift {
    pub opcode: u8,
    pub s_flag: BitState,
    pub rn: RegisterName,
    pub rd: RegisterName,
    pub shift_imm: u8,
    pub shift: u8,
    pub rm: RegisterName,
}

impl From<&Instruction> for DataProcessingImmediateShift {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 21) & 0b1111).unwrap();
        let s_flag = BitState::from(instruction_val >> 20);
        let rn = RegisterName::from((instruction_val >> 16) & 0b1111);
        let rd = RegisterName::from((instruction_val >> 12) & 0b1111);
        let shift_amount = u8::try_from((instruction_val >> 7) & 0b1_1111).unwrap();
        let shift = u8::try_from((instruction_val >> 5) & 0b11).unwrap();
        let rm = RegisterName::from(instruction_val & 0b1111);

        Self{opcode, s_flag, rn, rd, shift_imm: shift_amount, shift, rm}
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DataProcessingImmediateShift,
        BitState,
        Instruction,
        RegisterName,
    };

    #[test]
    fn test_from() {
        let instruction = Instruction::from(0b0000_000_1010_1_1010_0101_11100_10_0_1001);
        let value = DataProcessingImmediateShift::from(&instruction);

        let expected_value = DataProcessingImmediateShift {
            opcode: 0b1010,
            s_flag: BitState::Set,
            rn: RegisterName::R10,
            rd: RegisterName::R5,
            shift_imm: 0b11100,
            shift: 0b10,
            rm: RegisterName::R9,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
