use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlxSuffix {
    offset: u16,
}

impl<'a> From<DecodeData<'a>> for BlxSuffix {
    fn from(data: DecodeData<'a>) -> Self {
        let offset = u16::try_from((data.instruction.val >> 1) & 0b11_1111_1111).unwrap();
        Self { offset }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BlxSuffix,
        DecodeData,
    };

    use crate::{
        NintendoDS,
        cpus::general::Instruction,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b11101_10_1010_1010_0);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);
        
        let value = BlxSuffix::from(data);

        let expected_value = BlxSuffix {
            offset: 0b10_1010_1010,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
