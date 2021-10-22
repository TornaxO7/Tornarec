use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchAndBranchWithLink {
    l_flag: BitState,
    offset: u32,
}

impl<'a> From<DecodeData<'a>> for BranchAndBranchWithLink {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

        let l_flag = BitState::from(instruction_val >> 24);
        let offset = instruction_val & 0b1111_1111_1111_1111_1111_1111;

        Self { l_flag, offset }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        BranchAndBranchWithLink,
        Instruction,
    };

    #[test]
    fn from_branch() {
        let branch_instruction = Instruction::from(0b0000_101_0_1111_1111_1111_1111_0000_0000);
        let value = BranchAndBranchWithLink::from(&branch_instruction);

        let expected_value = BranchAndBranchWithLink {
            l_flag: BitState::Unset,
            offset: 0b1111_1111_1111_1111_0000_0000,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }

    #[test]
    fn from_branch_with_link() {
        let branch_with_link = Instruction::from(0b0000_101_1_1111_1111_1111_1111_0000_0000);
        let value = BranchAndBranchWithLink::from(&branch_with_link);

        let expected_value = BranchAndBranchWithLink {
            l_flag: BitState::Set,
            offset: 0b1111_1111_1111_1111_0000_0000,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
