use crate::cpus::general::{
    instruction::Instruction,
    register::RegisterName,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Miscellaneous2 {
    op1: u8,
    rn: RegisterName,
    rd: RegisterName,
    rs: RegisterName,
    op2: u8,
    rm: RegisterName,
}

impl From<&Instruction> for Miscellaneous2 {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let op1 = u8::try_from((instruction_val >> 21) & 0b11).unwrap();
        let rn = RegisterName::from((instruction_val >> 16) & 0b1111);
        let rd = RegisterName::from((instruction_val >> 12) & 0b1111);
        let rs = RegisterName::from((instruction_val >> 8) & 0b1111);
        let op2 = u8::try_from((instruction_val >> 5) & 0b11).unwrap();
        let rm = RegisterName::from(instruction_val & 0b1111);
        Self{op1, rn, rd, rs, op2, rm}
    }
}

#[cfg(test)]
mod tests {
    use super::{Miscellaneous2, Instruction, RegisterName};

    #[test]
    fn test_from() {
        let instruction = Instruction::from(0b0000_00010_11_0_1010_0101_1001_0_11_1_1111);
        let value = Miscellaneous2::from(&instruction);

        let expected_value = Miscellaneous2 {
            op1: 0b11,
            rn: RegisterName::R10,
            rd: RegisterName::R5,
            rs: RegisterName::R9,
            op2: 0b11,
            rm: RegisterName::R15,
        };

        assert_eq!(value, expected_value);
    }
}
