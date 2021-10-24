use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddToSpOrPc {
    sp: BitState,
    rd: u8,
    immediate: u8,
}

impl<'a> From<DecodeData<'a>> for AddToSpOrPc {
    fn from(data: DecodeData<'a>) -> Self {
        let sp = BitState::from(data.instruction.val >> 11);
        let rd = u8::try_from((data.instruction.val >> 8) & 0b111).unwrap();
        let immediate = u8::try_from(data.instruction.val & 0b1111_1111).unwrap();
        Self { sp, rd, immediate }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AddToSpOrPc,
        BitState,
        DecodeData,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b1011_1_101_1100_1000,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = AddToSpOrPc::from(data);

        let expected_value = AddToSpOrPc {
            sp: BitState::Set,
            rd: 0b0101,
            immediate: 0b1100_1000,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
