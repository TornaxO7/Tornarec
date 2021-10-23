use crate::cpus::general::{
    instruction::decode::DecodeData,
    register::NormalizedRegister,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Miscellaneous2 {
    op1: u8,
    rn: NormalizedRegister,
    rd: NormalizedRegister,
    rs: NormalizedRegister,
    op2: u8,
    rm: NormalizedRegister,
}

impl From<DecodeData> for Miscellaneous2 {
    fn from(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();

        let op1 = u8::try_from((instruction_val >> 21) & 0b11).unwrap();
        let rn = NormalizedRegister::from((instruction_val >> 16) & 0b1111);
        let rd = NormalizedRegister::from((instruction_val >> 12) & 0b1111);
        let rs = NormalizedRegister::from((instruction_val >> 8) & 0b1111);
        let op2 = u8::try_from((instruction_val >> 5) & 0b11).unwrap();
        let rm = NormalizedRegister::from(instruction_val & 0b1111);
        Self{op1, rn, rd, rs, op2, rm}
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Miscellaneous2,
        NormalizedRegister,
        DecodeData,
    };

    use crate::{
        cpus::general::{
            register::RegisterName,
            Instruction,
        },
        NintendoDS,
    };

    #[test]
    fn test_from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b0000_00010_11_0_1010_0101_1001_0_11_1_1111);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = Miscellaneous2::from(data);

        let expected_value = Miscellaneous2 {
            op1: 0b11,
            rn: NormalizedRegister::from(RegisterName::R10),
            rd: NormalizedRegister::from(RegisterName::R5),
            rs: NormalizedRegister::from(RegisterName::R9),
            op2: 0b11,
            rm: NormalizedRegister::from(RegisterName::R15),
        };

        assert_eq!(value, expected_value);
    }
}
