use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
    register::NormalizedRegister,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreWordByteImmediateOffset {
    b_flag: BitState,
    l_flag: BitState,
    offset: u8,
    rn: NormalizedRegister,
    rd: NormalizedRegister,
}

impl<'a> From<DecodeData<'a>> for LoadStoreWordByteImmediateOffset {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

        let b_flag = BitState::from(instruction_val >> 12);
        let l_flag = BitState::from(instruction_val >> 11);
        let offset = u8::try_from((instruction_val >> 6) & 0b1_1111).unwrap();
        let rn = NormalizedRegister::from((instruction_val >> 3) & 0b111);
        let rd = NormalizedRegister::from(instruction_val & 0b111);
        Self {b_flag, l_flag, offset, rn, rd}
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadStoreWordByteImmediateOffset, Instruction, BitState, NormalizedRegister};

    use crate::cpus::general::register::RegisterName;

    #[test]
    fn from() {
        let instruction = Instruction::from(0b011_1_0_11100_101_010);
        let value = LoadStoreWordByteImmediateOffset::from(&instruction);

        let expected_value = LoadStoreWordByteImmediateOffset {
            b_flag: BitState::Set,
            l_flag: BitState::Unset,
            offset: 0b11100,
            rn: NormalizedRegister::from(RegisterName::R5),
            rd: NormalizedRegister::from(RegisterName::R2),
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
