use super::error::MiscellaneousError;

use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveImmediateToStatusRegister {
    pub r_flag: BitState,
    pub mask: u8,
    pub rotate: u8,
    pub immediate: u8,
}

impl<'a> From<DecodeData<'a>> for MoveImmediateToStatusRegister {
    fn from(data: DecodeData<'a>) -> Self {
        let r_flag = BitState::from(data.instruction.val >> 22);
        let mask = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let sbo = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let rotate = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let immediate = u8::try_from(data.instruction.val & 0b1111_1111).unwrap();

        if sbo != 0b1111 {
            unreachable!("{}", MiscellaneousError::SBOConflict(data.instruction.val));
        }

        Self {
            r_flag,
            mask,
            rotate,
            immediate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DecodeData,
        MoveImmediateToStatusRegister,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b0000_00110_1_10_1010_1111_1001_1111_0000,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = MoveImmediateToStatusRegister::from(data);

        let expected_value = MoveImmediateToStatusRegister {
            r_flag: BitState::Set,
            mask: 0b1010,
            rotate: 0b1001,
            immediate: 0b1111_0000,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }

    #[test]
    #[should_panic]
    fn from_invalid_sbo() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b0000_00110_1_10_1010_0110_1001_1111_0000,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        MoveImmediateToStatusRegister::from(data);
    }
}
