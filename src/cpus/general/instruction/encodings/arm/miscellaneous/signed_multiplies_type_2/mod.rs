mod signed_multiplies_opcode;

pub use signed_multiplies_opcode::SignedMultipliesOpcode;

use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedMultipliesType2 {
    pub opcode: SignedMultipliesOpcode,
    pub rd: u8,
    pub rn: u8,
    pub rs: u8,
    pub y: BitState,
    pub x: BitState,
    pub rm: u8,
}

impl<'a> From<DecodeData<'a>> for SignedMultipliesType2 {
    fn from(data: DecodeData<'a>) -> Self {
        let opcode = SignedMultipliesOpcode::from(&data.instruction);
        let rd = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let rn = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let rs = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let y = BitState::from(data.instruction.val >> 6);
        let x = BitState::from(data.instruction.val >> 5);
        let rm = u8::try_from(data.instruction.val & 0b1111).unwrap();

        Self {
            opcode,
            rd,
            rn,
            rs,
            y,
            x,
            rm,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    use super::{
        BitState,
        DecodeData,
        SignedMultipliesType2,
        SignedMultipliesOpcode,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let data = {
            let instruction = Instruction {
                val: 0b0000_00010_110_1111_1110_1100_1110_1000,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = SignedMultipliesType2::from(data);
        let expected_value = SignedMultipliesType2 {
            opcode: SignedMultipliesOpcode::SMUL,
            rd: 0b1111,
            rn: 0b1110,
            rs: 0b1100,
            y: BitState::Set,
            x: BitState::Set,
            rm: 0b1000,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
        );
    }
}
