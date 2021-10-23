use crate::cpus::general::{
    instruction::decode::DecodeData,
    register::RegisterName,
    BitState,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchExchangeInstructionSet {
    l_flag: BitState,
    h2: BitState,
    rm: RegisterName,
}

impl<'a> From<DecodeData<'a>> for BranchExchangeInstructionSet {
    fn from(data: DecodeData<'a>) -> Self {
        let l_flag = BitState::from(data.instruction.val >> 7);
        let h2 = BitState::from(data.instruction.val >> 6);
        let rm = RegisterName::from((data.instruction.val >> 3) & 0b111);
        Self { l_flag, h2, rm }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        BranchExchangeInstructionSet,
        DecodeData,
        RegisterName,
    };

    use crate::{
        NintendoDS,
        cpus::general::Instruction,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b010001_11_1_0_101_111);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = BranchExchangeInstructionSet::from(data);

        let expected_value = BranchExchangeInstructionSet {
            l_flag: BitState::Set,
            h2: BitState::Unset,
            rm: RegisterName::R5,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
