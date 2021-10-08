use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingRegisterShift {
    opcode: u8,
    s_flag: BitState,
    rn: u8,
    rd: u8,
    rs: u8,
    shift: u8,
    rm: u8,
}

impl From<&Instruction> for DataProcessingRegisterShift {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 21) & 0b1111).unwrap();
        let s_flag = BitState::from(instruction_val >> 20);
        let rn = u8::try_from((instruction_val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((instruction_val >> 12) & 0b1111).unwrap();
        let rs = u8::try_from((instruction_val >> 8) & 0b1111).unwrap();
        let shift = u8::try_from((instruction_val >> 5) & 0b11).unwrap();
        let rm = u8::try_from(instruction_val & 0b1111).unwrap();

        Self{opcode, s_flag, rn, rd, rs, shift, rm}
    }
}

#[cfg(test)]
mod tests {
    use super::{DataProcessingRegisterShift, Instruction, BitState};

    #[test]
    fn test_from() {
        let instruction = Instruction::from(0b0000_000_1111_1_1010_0101_0110_0_11_1_1001);
        let value = DataProcessingRegisterShift::from(&instruction);

        let expected_value = DataProcessingRegisterShift {
            opcode: 0b1111,
            s_flag: BitState::Set,
            rn: 0b1010,
            rd: 0b0101,
            rs: 0b0110,
            shift: 0b11,
            rm: 0b1001,
        };

        assert_eq!(value, expected_value);
    }
}
