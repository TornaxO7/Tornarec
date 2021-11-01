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
pub struct MoveRegisterToStatusRegister {
    r_flag: BitState,
    mask: u8,
    rm: u8,
}

impl<'a> From<DecodeData<'a>> for MoveRegisterToStatusRegister {
    fn from(data: DecodeData<'a>) -> Self {
        let r_flag = BitState::from(data.instruction.val >> 22);
        let mask = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let sbo = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let sbz = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let rm = u8::try_from(data.instruction.val & 0b1111).unwrap();

        if sbo != 0b1111 {
            unreachable!("{}", MiscellaneousError::SBOConflict(data.instruction.val));
        } else if sbz != 0 {
            unreachable!("{}", MiscellaneousError::SBZConflict(data.instruction.val));
        }

        Self { r_flag, mask, rm }
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
        MoveRegisterToStatusRegister,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let data = {
            let instruction = Instruction {
                val: 0b0000_00010_110_1101_1111_0000_0000_1100,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = MoveRegisterToStatusRegister::from(data);
        let expected_value = MoveRegisterToStatusRegister {
            r_flag: BitState::Set,
            mask: 0b1101,
            rm: 0b1100,
        };

        assert_eq!(expected_value, value, "{:#?} {:#?}", &expected_value, &value);
    }
}
