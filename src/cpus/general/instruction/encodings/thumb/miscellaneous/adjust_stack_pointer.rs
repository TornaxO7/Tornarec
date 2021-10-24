use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdjustStackPointer {
    opc: BitState,
    immediate: u8,
}

impl<'a> From<DecodeData<'a>> for AdjustStackPointer {
    fn from(data: DecodeData<'a>) -> Self {
        let opc = BitState::from(data.instruction.val >> 7);
        let immediate = u8::try_from(data.instruction.val & 0b111_1111).unwrap();
        Self { opc, immediate }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AdjustStackPointer,
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
            val: 0b1011_0000_1_100_1000,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = AdjustStackPointer::from(data);

        let expected_value = AdjustStackPointer {
            opc: BitState::Set,
            immediate: 0b100_1000,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
