use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoprocessorDataProcessing {
    opcode1: u8,
    crn: u8,
    crd: u8,
    cp_num: u8,
    opcode2: u8,
    crm: u8,
}

impl<'a> From<DecodeData<'a>> for CoprocessorDataProcessing {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

        let opcode1 = u8::try_from((instruction_val >> 20) & 0b1111).unwrap();
        let crn = u8::try_from((instruction_val >> 16) & 0b1111).unwrap();
        let crd = u8::try_from((instruction_val >> 12) & 0b1111).unwrap();
        let cp_num = u8::try_from((instruction_val >> 8) & 0b1111).unwrap();
        let opcode2 = u8::try_from((instruction_val >> 5) & 0b111).unwrap();
        let crm = u8::try_from(instruction_val & 0b1111).unwrap();
        Self {
            opcode1,
            crn,
            crd,
            cp_num,
            opcode2,
            crm,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CoprocessorDataProcessing,
        Instruction,
    };

    #[test]
    fn from() {
        let instruction = Instruction::from(0b0000_1110_1111_1110_1100_1000_111_0_1111);
        let value = CoprocessorDataProcessing::from(&instruction);

        let expected_value = CoprocessorDataProcessing {
            opcode1: 0b1111,
            crn: 0b1110,
            crd: 0b1100,
            cp_num: 0b1000,
            opcode2: 0b111,
            crm: 0b1111,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
