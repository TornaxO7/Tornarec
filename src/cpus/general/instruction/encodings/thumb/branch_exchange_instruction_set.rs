use crate::cpus::general::{
    instruction::Instruction,
    BitState,
    register::RegisterName,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchExchangeInstructionSet {
    l_flag: BitState,
    h2: BitState,
    rm: RegisterName,
}

impl From<&Instruction> for BranchExchangeInstructionSet {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let l_flag = BitState::from(instruction_val >> 7);
        let h2 = BitState::from(instruction_val >> 6);
        let rm = RegisterName::from((instruction_val >> 3) & 0b111);
        Self {l_flag, h2, rm}
    }
}

#[cfg(test)]
mod tests {
    use super::{BranchExchangeInstructionSet, Instruction, BitState, RegisterName};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b010001_11_1_0_101_111);
        let value = BranchExchangeInstructionSet::from(&instruction);

        let expected_value = BranchExchangeInstructionSet {
            l_flag: BitState::Set,
            h2: BitState::Unset,
            rm: RegisterName::R5,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
