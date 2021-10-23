use crate::cpus::general::{
    instruction::decode::DecodeData,
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
    rm: u8,
    rd_rn: u8,
}

impl<'a> From<DecodeData<'a>> for SpecialDataProcessing {
    fn from(data: DecodeData<'a>) -> Self {
        let opcode = u8::try_from((data.instruction.val >> 8) & 0b11).unwrap();
        let h1 = BitState::from(data.instruction.val >> 7);
        let h2 = BitState::from(data.instruction.val >> 6);
        let rm = u8::try_from((data.instruction.val >> 3) & 0b111).unwrap();
        let rd_rn = u8::try_from(data.instruction.val & 0b111).unwrap();
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
