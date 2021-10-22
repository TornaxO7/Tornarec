pub mod error;

use error::MoveImmediateToStatusRegisterError;

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
    r_flag: BitState,
    mask: u8,
    rotate: u8,
    immediate: u8,
}

impl<'a> From<DecodeData<'a>> for MoveImmediateToStatusRegister {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

        let r_flag = BitState::from(instruction_val >> 22);
        let mask = u8::try_from((instruction_val >> 16) & 0b1111).unwrap();
        let sbo = u8::try_from((instruction_val >> 12) & 0b1111).unwrap();
        let rotate = u8::try_from((instruction_val >> 8) & 0b1111).unwrap();
        let immediate = u8::try_from(instruction_val & 0b1111_1111).unwrap();

        if sbo != 0b1111 {
            unreachable!(
                "{}",
                MoveImmediateToStatusRegisterError::SBOConflict(instruction_val)
            );
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
        NintendoDS,
        cpus::general::Instruction,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b0000_00110_1_10_1010_1111_1001_1111_0000);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

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
        let instruction = Instruction::from(0b0000_00110_1_10_1010_0110_1001_1111_0000);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        MoveImmediateToStatusRegister::from(data);
    }
}
