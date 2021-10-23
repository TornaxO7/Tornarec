use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSubtractRegister {
    opc: BitState,
    rm:  u8,
    rn:  u8,
    rd:  u8,
}

impl<'a> From<DecodeData<'a>> for AddSubtractRegister {
    fn from(data: DecodeData<'a>) -> Self {
        let opc = BitState::from(data.instruction.val >> 9);
        let rm = u8::try_from((data.instruction.val >> 6) & 0b111).unwrap();
        let rn = u8::try_from((data.instruction.val >> 3) & 0b111).unwrap();
        let rd = u8::try_from(data.instruction.val & 0b111).unwrap();
        Self { opc, rm, rn, rd }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AddSubtractRegister,
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
            val: 0b000_11_0_1_111_110_100,
            .. Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = AddSubtractRegister::from(data);

        let expected_value = AddSubtractRegister {
            opc: BitState::Set,
            rm:  0b0111,
            rn:  0b0110,
            rd:  0b0100,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
