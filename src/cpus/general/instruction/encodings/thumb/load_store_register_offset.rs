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
        let instruction = Instruction::from(0b0101_111_110_100_101);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = LoadStoreRegisterOffset::from(data);

        let expected_value = LoadStoreRegisterOffset {
            opcode: 0b111,
            rm: NormalizedRegister::from(RegisterName::R6),
            rn: NormalizedRegister::from(RegisterName::R4),
            rd: NormalizedRegister::from(RegisterName::R5),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
