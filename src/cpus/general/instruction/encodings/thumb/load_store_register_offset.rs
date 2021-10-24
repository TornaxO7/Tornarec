use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreRegisterOffset {
    opcode: u8,
    rm: u8,
    rn: u8,
    rd: u8,
}

impl<'a> From<DecodeData<'a>> for LoadStoreRegisterOffset {
    fn from(data: DecodeData<'a>) -> Self {
        let opcode = u8::try_from((data.instruction.val >> 9) & 0b111).unwrap();
        let rm = u8::try_from((data.instruction.val >> 6) & 0b111).unwrap();
        let rn = u8::try_from((data.instruction.val >> 3) & 0b111).unwrap();
        let rd = u8::try_from(data.instruction.val & 0b111).unwrap();
        Self { opcode, rm, rn, rd }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DecodeData,
        LoadStoreRegisterOffset,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b0101_111_110_100_101,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = LoadStoreRegisterOffset::from(data);

        let expected_value = LoadStoreRegisterOffset {
            opcode: 0b111,
            rm: 0b0110,
            rn: 0b0100,
            rd: 0b0101,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
