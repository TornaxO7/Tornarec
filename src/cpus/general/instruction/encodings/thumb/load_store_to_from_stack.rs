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
pub struct LoadStoreToFromStack {
    l_flag: BitState,
    rd: NormalizedRegister,
    sp_relative_offset: u8,
}

impl<'a> From<DecodeData<'a>> for LoadStoreToFromStack {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

        let l_flag = BitState::from(instruction_val >> 11);
        let rd = NormalizedRegister::from((instruction_val >> 8) & 0b111);
        let sp_relative_offset = u8::try_from(instruction_val & 0b1111_1111).unwrap();
        Self {
            l_flag,
            rd,
            sp_relative_offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DecodeData,
        LoadStoreToFromStack,
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
        let instruction = Instruction::from(0b1001_1_101_1100_0011);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);
        
        let value = LoadStoreToFromStack::from(data);

        let expected_value = LoadStoreToFromStack {
            l_flag: BitState::Set,
            rd: NormalizedRegister::from(RegisterName::R5),
            sp_relative_offset: 0b1100_0011,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
