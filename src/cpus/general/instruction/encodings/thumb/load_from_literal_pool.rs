use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadFromLiteralPool {
    rd: u8,
    pc_relative_offset: u8,
}

impl<'a> From<DecodeData<'a>> for LoadFromLiteralPool {
    fn from(data: DecodeData<'a>) -> Self {
        let rd = u8::try_from((data.instruction.val >> 8) & 0b111).unwrap();
        let pc_relative_offset = u8::try_from(data.instruction.val & 0b1111_1111).unwrap();
        Self {
            rd,
            pc_relative_offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DecodeData,
        LoadFromLiteralPool,
    };

    use crate::{
        NintendoDS,
        cpus::general::Instruction,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b01001_111_1010_0101);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = LoadFromLiteralPool::from(data);

        let expected_value = LoadFromLiteralPool {
            rd: 0b111,
            pc_relative_offset: 0b1010_0101,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
