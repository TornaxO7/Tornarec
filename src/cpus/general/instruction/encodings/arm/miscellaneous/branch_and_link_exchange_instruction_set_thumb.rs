use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchAndLinkExchangeInstructionSetThumb {
    pub h_flag: BitState,
    pub signed_immed_24: u32,
}

impl<'a> From<DecodeData<'a>> for BranchAndLinkExchangeInstructionSetThumb {
    fn from(data: DecodeData<'a>) -> Self {
        let h_flag = BitState::from(data.instruction.val >> 24);
        let signed_immed_24 = data.instruction.val & 0b1111_1111_1111_1111_1111_1111;

        Self {
            h_flag,
            signed_immed_24,
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
        BranchAndLinkExchangeInstructionSetThumb,
        BitState,
        DecodeData,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let data = {
            let instruction = Instruction {
                val: 0b1111_101_1_1111_1111_1111_1111_1111_1111,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = BranchAndLinkExchangeInstructionSetThumb::from(data);
        let expected_value = BranchAndLinkExchangeInstructionSetThumb {
            h_flag: BitState::Set,
            signed_immed_24: 0b1111_1111_1111_1111_1111_1111,
        };

        assert_eq!(
            expected_value, value,
            "{:#?} {:#?}",
            &expected_value, &value
        );
    }
}
