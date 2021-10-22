use crate::cpus::general::{
    instruction::Instruction,
    register::NormalizedRegister,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Multiplies {
    op1: u8,
    rn: NormalizedRegister,
    rd: NormalizedRegister,
    rs: NormalizedRegister,
    rm: NormalizedRegister,
}

impl From<&Instruction> for Multiplies {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let op1 = u8::try_from((instruction_val >> 20) & 0b1111).unwrap();
        let rn = NormalizedRegister::from((instruction_val >> 16) & 0b1111);
        let rd = NormalizedRegister::from((instruction_val >> 12) & 0b1111);
        let rs = NormalizedRegister::from((instruction_val >> 8) & 0b1111);
        let rm = NormalizedRegister::from(instruction_val & 0b1111);
        Self{op1, rn, rd, rs, rm}
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Multiplies,
        Instruction,
        NormalizedRegister
    };

    use crate::cpus::general::register::RegisterName;

    #[test]
    fn test_from() {
        let instruction = Instruction::from(0b0000_0000_1111_1100_0011_1110_1001_0110);
        let value = Multiplies::from(&instruction);

        let expected_value = Multiplies {
            op1: 0b1111,
            rn: NormalizedRegister::from(RegisterName::R12),
            rd: NormalizedRegister::from(RegisterName::R3),
            rs: NormalizedRegister::from(RegisterName::R14),
            rm: NormalizedRegister::from(RegisterName::R6),
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
