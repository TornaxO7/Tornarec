use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchAndBranchWithLink {
    pub l_flag: BitState,
    pub offset: u32,
}

impl<'a> From<DecodeData<'a>> for BranchAndBranchWithLink {
    fn from(data: DecodeData<'a>) -> Self {
        let l_flag = BitState::from(data.instruction.val >> 24);
        let offset = data.instruction.val & 0b1111_1111_1111_1111_1111_1111;

        Self { l_flag, offset }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        BranchAndBranchWithLink,
        DecodeData,
    };

    use crate::{
        NintendoDS,
        cpus::general::Instruction,
    };

    #[test]
    fn from_branch() {
        let nds = NintendoDS::default();
        let branch_instruction = Instruction::from(0b0000_101_0_1111_1111_1111_1111_0000_0000);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &branch_instruction);

        let value = BranchAndBranchWithLink::from(data);

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
        let nds = NintendoDS::default();
        let branch_with_link = Instruction::from(0b0000_101_1_1111_1111_1111_1111_0000_0000);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &branch_with_link);

        let value = BranchAndBranchWithLink::from(data);

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
