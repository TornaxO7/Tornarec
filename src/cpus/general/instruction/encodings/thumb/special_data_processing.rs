use crate::cpus::general::{
    instruction::decode::DecodeData,
    register::NormalizedRegister,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialDataProcessing {
    opcode: u8,
    h1: BitState,
    h2: BitState,
    rm: NormalizedRegister,
    rd_rn: NormalizedRegister,
}

impl From<DecodeData> for SpecialDataProcessing {
    fn from(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 8) & 0b11).unwrap();
        let h1 = BitState::from(instruction_val >> 7);
        let h2 = BitState::from(instruction_val >> 6);
        let rm = NormalizedRegister::from((instruction_val >> 3) & 0b111);
        let rd_rn = NormalizedRegister::from(instruction_val & 0b111);
        Self {
            opcode,
            h1,
            h2,
            rm,
            rd_rn,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DecodeData,
        NormalizedRegister,
        SpecialDataProcessing,
    };

    use crate::{
        cpus::general::{
            register::RegisterName,
            Instruction,
        },
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b010001_11_1_0_101_010);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = SpecialDataProcessing::from(data);

        let expected_value = SpecialDataProcessing {
            opcode: 0b11,
            h1: BitState::Set,
            h2: BitState::Unset,
            rm: NormalizedRegister::from(RegisterName::R5),
            rd_rn: NormalizedRegister::from(RegisterName::R2),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
