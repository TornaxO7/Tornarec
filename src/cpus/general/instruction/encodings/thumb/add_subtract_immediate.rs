use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSubtractImmediate {
    opc: BitState,
    immediate: u8,
    rn: u8,
    rd: u8,
}

impl<'a> From<DecodeData<'a>> for AddSubtractImmediate {
    fn from(data: DecodeData<'a>) -> Self {
        let opc = BitState::from(data.instruction.val >> 9);
        let immediate = u8::try_from((data.instruction.val >> 6) & 0b111).unwrap();
        let rn = u8::try_from((data.instruction.val >> 3) & 0b111).unwrap();
        let rd = u8::try_from(data.instruction.val & 0b111).unwrap();
        Self {
            opc,
            immediate,
            rn,
            rd,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AddSubtractImmediate,
        BitState,
        DecodeData,
    };

    use crate::{
        NintendoDS,
        cpus::general::Instruction,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b000_111_1_111_110_100,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = AddSubtractImmediate::from(data);

        let expected_value = AddSubtractImmediate {
            opc: BitState::Set,
            immediate: 0b111,
            rn: 0b0110,
            rd: 0b0100,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
