use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingImmediateShift {
    opcode: u8,
    s_flag: BitState,
    rn: u8,
    rd: u8,
    shift_amount: u8,
    shift: u8,
    rm: u8,
}

impl From<&Instruction> for DataProcessingImmediateShift {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 21) & 0b1111).unwrap();
        let s_flag = BitState::from(instruction_val >> 20);
        let rn = u8::try_from((instruction_val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((instruction_val >> 12) & 0b1111).unwrap();
        let shift_amount = u8::try_from((instruction_val >> 7) & 0b1_1111).unwrap();
        let shift = u8::try_from((instruction_val >> 5) & 0b11).unwrap();
        let rm = u8::try_from(instruction_val & 0b1111).unwrap();

        Self{opcode, s_flag, rn, rd, shift_amount, shift, rm}
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DataProcessingImmediateShift,
        BitState,
        Instruction,
    };

    #[test]
    fn test_from() {
        let instruction = Instruction::from(0b0000_000_1010_1_1010_0101_11100_10_0_1001);
        let value = DataProcessingImmediateShift::from(&instruction);

        let expected_value = DataProcessingImmediateShift {
            opcode: 0b1010,
            s_flag: BitState::Set,
            rn: 0b1010,
            rd: 0b0101,
            shift_amount: 0b11100,
            shift: 0b10,
            rm: 0b1001,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
