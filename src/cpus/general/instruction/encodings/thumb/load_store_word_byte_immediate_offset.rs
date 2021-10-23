use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreWordByteImmediateOffset {
    b_flag: BitState,
    l_flag: BitState,
    offset: u8,
    rn: u8,
    rd: u8,
}

impl<'a> From<DecodeData<'a>> for LoadStoreWordByteImmediateOffset {
    fn from(data: DecodeData<'a>) -> Self {
        let b_flag = BitState::from(data.instruction.val >> 12);
        let l_flag = BitState::from(data.instruction.val >> 11);
        let offset = u8::try_from((data.instruction.val >> 6) & 0b1_1111).unwrap();
        let rn = u8::try_from((data.instruction.val >> 3) & 0b111).unwrap();
        let rd = u8::try_from(data.instruction.val & 0b111).unwrap();
        Self {
            b_flag,
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
        LoadStoreWordByteImmediateOffset,
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
        let instruction = Instruction::from(0b011_1_0_11100_101_010);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = LoadStoreWordByteImmediateOffset::from(data);

        let expected_value = LoadStoreWordByteImmediateOffset {
            b_flag: BitState::Set,
            l_flag: BitState::Unset,
            offset: 0b11100,
            rn: NormalizedRegister::from(RegisterName::R5),
            rd: NormalizedRegister::from(RegisterName::R2),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
