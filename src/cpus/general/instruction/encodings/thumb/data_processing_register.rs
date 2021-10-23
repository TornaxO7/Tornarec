use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingRegister {
    opcode: u8,
    rm_rs: u8,
    rd_rn: u8,
}

impl<'a> From<DecodeData<'a>> for DataProcessingRegister {
    fn from(data: DecodeData<'a>) -> Self {
        let opcode = u8::try_from((data.instruction.val >> 6) & 0b1111).unwrap();
        let rm_rs = u8::try_from((data.instruction.val >> 3) & 0b111).unwrap();
        let rd_rn = u8::try_from(data.instruction.val & 0b111).unwrap();
        Self {
            opcode,
            rm_rs,
            rd_rn,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DataProcessingRegister,
        DecodeData,
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
        let instruction = Instruction::from(0b010000_1111_101_010);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = DataProcessingRegister::from(data);

        let expected_value = DataProcessingRegister {
            opcode: 0b1111,
             rm_rs: NormalizedRegister::from(RegisterName::R5),
             rd_rn: NormalizedRegister::from(RegisterName::R2),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
