use super::error::MiscellaneousError;

use std::convert::{
    From,
    TryFrom,
};

use crate::cpus::general::instruction::decode::DecodeData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchExchangeInstructionSetJava {
    rm: u8,
}

impl<'a> From<DecodeData<'a>> for BranchExchangeInstructionSetJava {
    fn from(data: DecodeData<'a>) -> Self {
        let sbo1 = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let sbo2 = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let sbo3 = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let rm = u8::try_from(data.instruction.val & 0b1111).unwrap();

        if sbo1 != 0b1111 || sbo2 != 0b1111 || sbo3 != 0b1111 {
            unreachable!("{}", MiscellaneousError::SBOConflict(data.instruction.val));
        }

        Self { rm }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    use super::{
        BranchExchangeInstructionSetJava,
        DecodeData,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let data = {
            let instruction = Instruction {
                val: 0b0000_00010_010_1111_1111_1111_0010_1001,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = BranchExchangeInstructionSetJava::from(data);
        let expected_value = BranchExchangeInstructionSetJava { rm: 0b1001 };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
        );
    }
}
