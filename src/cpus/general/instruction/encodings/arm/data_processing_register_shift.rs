use crate::cpus::general::{
    bit_state::BitState,
    instruction::{
        Instruction,
        encodings::encoding_fields::DataProcessingInstruction,
    },
    register::NormalizedRegister,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingRegisterShift {
    opcode: DataProcessingInstruction,
    s_flag: BitState,
    rn: NormalizedRegister,
    rd: NormalizedRegister,
    rs: NormalizedRegister,
    shift: u8,
    rm: NormalizedRegister,
}

impl From<&Instruction> for DataProcessingRegisterShift {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opcode = DataProcessingInstruction::from((instruction_val >> 21) & 0b1111);
        let s_flag = BitState::from(instruction_val >> 20);
        let rn = NormalizedRegister::from((instruction_val >> 16) & 0b1111);
        let rd = NormalizedRegister::from((instruction_val >> 12) & 0b1111);
        let rs = NormalizedRegister::from((instruction_val >> 8) & 0b1111);
        let shift = u8::try_from((instruction_val >> 5) & 0b11).unwrap();
        let rm = NormalizedRegister::from(instruction_val & 0b1111);

        Self{opcode, s_flag, rn, rd, rs, shift, rm}
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DataProcessingRegisterShift,
        Instruction,
        BitState,
        NormalizedRegister,
        DataProcessingInstruction
    };

    use crate::cpus::general::register::RegisterName;

    #[test]
    fn test_from() {
        let instruction = Instruction::from(0b0000_000_1111_1_1010_0101_0110_0_11_1_1001);
        let value = DataProcessingRegisterShift::from(&instruction);

        let expected_value = DataProcessingRegisterShift {
            opcode: DataProcessingInstruction::MVN,
            s_flag: BitState::Set,
            rn: NormalizedRegister::from(RegisterName::R10),
            rd: NormalizedRegister::from(RegisterName::R5),
            rs: NormalizedRegister::from(RegisterName::R6),
            shift: 0b11,
            rm: NormalizedRegister::from(RegisterName::R9),
        };

        assert_eq!(value, expected_value);
    }
}
