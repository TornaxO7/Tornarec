use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchExchangeInstructionSet {
    l_flag: BitState,
    h2: BitState,
    rm: u8,
}

impl<'a> From<DecodeData<'a>> for BranchExchangeInstructionSet {
    fn from(data: DecodeData<'a>) -> Self {
        let l_flag = BitState::from(data.instruction.val >> 7);
        let h2 = BitState::from(data.instruction.val >> 6);
        let rm = u8::try_from((data.instruction.val >> 3) & 0b111).unwrap();
        Self { l_flag, h2, rm }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        BranchExchangeInstructionSet,
        DecodeData,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b010001_11_1_0_101_111,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = BranchExchangeInstructionSet::from(data);

        let expected_value = BranchExchangeInstructionSet {
            l_flag: BitState::Set,
            h2: BitState::Unset,
            rm: 0b0101,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
