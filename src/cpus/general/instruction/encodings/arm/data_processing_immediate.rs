use crate::cpus::general::{
    instruction::{
        Instruction,
        encodings::encoding_fields::DataProcessingInstruction,
    },
    bit_state::BitState,
    register::NormalizedRegister,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingImmediate {
    opcode: DataProcessingInstruction,
    s_flag: BitState,
    rn: NormalizedRegister,
    rd: NormalizedRegister,
    rotate: u8,
    immediate: u8,
}

impl From<&Instruction> for DataProcessingImmediate {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opcode = DataProcessingInstruction::from((instruction_val >> 21) & 0b1111);
        let s_flag = BitState::from(instruction_val >> 20);
        let rn = NormalizedRegister::from((instruction_val >> 16) & 0b1111);
        let rd = NormalizedRegister::from((instruction_val >> 12) & 0b1111);
        let rotate = u8::try_from((instruction_val >> 8) & 0b1111).unwrap();
        let immediate = u8::try_from(instruction_val & 0b1111_1111).unwrap();
        Self{opcode, s_flag, rn, rd, rotate, immediate}
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DataProcessingImmediate,
        Instruction,
        BitState,
        NormalizedRegister,
        DataProcessingInstruction,
    };

    use crate::cpus::general::register::RegisterName;

    #[test]
    fn from() {
        let instruction = Instruction::from(0b0000_001_1111_1_1100_0011_1010_0011_1011);
        let value = DataProcessingImmediate::from(&instruction);

        let expected_value = DataProcessingImmediate {
            opcode: DataProcessingInstruction::MVN,
            s_flag: BitState::Set,
            rn: NormalizedRegister::from(RegisterName::R12),
            rd: NormalizedRegister::from(RegisterName::R3),
            rotate: 0b1010,
            immediate: 0b0011_1011,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
