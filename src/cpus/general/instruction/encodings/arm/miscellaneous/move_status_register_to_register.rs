use super::error::MiscellaneousError;

use crate::cpus::general::{
    instruction::decode::DecodeData,
    register::RegisterName,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveStatusRegisterToRegister {
    pub r_flag: BitState,
    pub rd_reg: RegisterName,
}

impl<'a> From<DecodeData<'a>> for MoveStatusRegisterToRegister {
    fn from(data: DecodeData<'a>) -> Self {
        let r_flag = BitState::from(data.instruction.val >> 22);
        let sbo = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let rd_reg = RegisterName::from((data.instruction.val >> 12) & 0b1111);
        let sbz1 = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let sbz2 = u8::try_from(data.instruction.val & 0b1111).unwrap();

        if sbo != 0b1111 {
            unreachable!("{}", MiscellaneousError::SBOConflict(data.instruction.val));
        } else if sbz1 != 0 || sbz2 != 0 {
            unreachable!("{}", MiscellaneousError::SBZConflict(data.instruction.val));
        }

        Self { r_flag, rd_reg }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DecodeData,
        MoveStatusRegisterToRegister,
        RegisterName,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b0000_00010_100_1111_1101_0000_0000_0000,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = MoveStatusRegisterToRegister::from(data);

        let expected_value = MoveStatusRegisterToRegister {
            r_flag: BitState::Set,
            rd_reg: RegisterName::R13,
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
            val: 0b0000_00010_100_1101_1111_0000_0000_0000,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let _ = MoveStatusRegisterToRegister::from(data);
    }
}
