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
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction{
            val: 0b010000_1111_101_010,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = DataProcessingRegister::from(data);

        let expected_value = DataProcessingRegister {
            opcode: 0b1111,
             rm_rs: 0b0101,
             rd_rn: 0b0010,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
