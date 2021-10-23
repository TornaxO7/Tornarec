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
pub struct LoadStoreHalfwordImmediateOffset {
    l_flag: BitState,
    offset: u8,
    rn: NormalizedRegister,
    rd: NormalizedRegister,
}

impl From<DecodeData> for LoadStoreHalfwordImmediateOffset {
    fn from(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();

        let l_flag = BitState::from(instruction_val >> 11);
        let offset = u8::try_from((instruction_val >> 6) & 0b1_1111).unwrap();
        let rn = NormalizedRegister::from((instruction_val >> 3) & 0b111);
        let rd = NormalizedRegister::from(instruction_val & 0b111);
        Self {
            l_flag,
            offset,
            rn,
            rd,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DecodeData,
        LoadStoreHalfwordImmediateOffset,
        NormalizedRegister,
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
        let instruction = Instruction::from(0b1000_1_10101_110_100);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = LoadStoreHalfwordImmediateOffset::from(data);

        let expected_value = LoadStoreHalfwordImmediateOffset {
            l_flag: BitState::Set,
            offset: 0b10101,
            rn: NormalizedRegister::from(RegisterName::R6),
            rd: NormalizedRegister::from(RegisterName::R4),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
