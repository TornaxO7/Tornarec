use super::error::MiscellaneousError;

use std::convert::{From, TryFrom};

use crate::cpus::general::instruction::decode::DecodeData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchExchangeInstructionSetThumb {
    rm: u8,
}

impl<'a> From<DecodeData<'a>> for BranchExchangeInstructionSetThumb {
    fn from(data: DecodeData<'a>) -> Self {
        let rm = u8::try_from(data.instruction.val & 0b1111).unwrap();
        let sbo1 = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let sbo2 = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let sbo3 = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();

        if sbo1 != 0b1111 || sbo2 != 0b1111 || sbo3 != 0b1111 {
            unreachable!("{}", MiscellaneousError::SBOConflict(data.instruction.val)); 
        }

        Self {
            rm,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{NintendoDS, cpus::general::Instruction};

    use super::{DecodeData, BranchExchangeInstructionSetThumb};

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let data = {
            let instruction = Instruction {
                val: 0b0000_00010_010_1111_1111_1111_0001_1101,
                .. Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = BranchExchangeInstructionSetThumb::from(data);
        let expected_value = BranchExchangeInstructionSetThumb {
            rm: 0b1101,
        };

        assert_eq!(expected_value, value, "{:#?} {:#?}", &expected_value, &value);
    }
}
