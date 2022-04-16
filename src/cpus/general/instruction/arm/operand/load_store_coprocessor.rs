use crate::{
    cpus::general::instruction::arm::{
        types::Register,
        BitState,
    },
    ram::Word,
};

use super::ArmOperand;

use std::convert::TryFrom;

pub fn get_ldc_operand(value: Word) -> ArmOperand {
    let u = BitState::new(value, 23);
    let n = BitState::new(value, 22);
    let rn = Register::new(value, 16, 0b1111);
    let crd = Register::new(value, 12, 0b1111);
    let cp_num = u8::try_from((value >> 8) & 0b1111).unwrap();
    let immed8 = u8::try_from(value & 0b1111_1111).unwrap();

    ArmOperand::LDC {
        u,
        n,
        rn,
        crd,
        cp_num,
        immed8,
        mode: LoadStoreCoprocessorMode::from(value),
    }
}

/// Also called "Addressing Mode 5" (page 489)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadStoreCoprocessorMode {
    ImmediateOffset,
    ImmediatePreIndexed,
    ImmediatePostIndexed,
    Unindexed,
}

impl From<Word> for LoadStoreCoprocessorMode {
    fn from(value: Word) -> Self {
        let p = BitState::new(value, 24);
        let w = BitState::new(value, 21);

        match (p, w) {
            (BitState::UNSET, BitState::UNSET) => Self::Unindexed,
            (BitState::UNSET, BitState::SET) => Self::ImmediatePostIndexed,
            (BitState::SET, BitState::UNSET) => Self::ImmediateOffset,
            (BitState::SET, BitState::SET) => Self::ImmediatePreIndexed,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpus::general::instruction::arm::operand::load_store_coprocessor::LoadStoreCoprocessorMode;

    #[test]
    fn test_load_store_coprocessor_mode() {
        let unindexed = 0b0000_1100_1101_1111_1111_1111_1111_1111;
        let immediate_post_indexed = 0b0000_1100_1111_1111_1111_1111_1111_1111;
        let immediate_pre_indexed = 0b0000_1101_1111_1111_1111_1111_1111_1111;
        let immediate_offset = 0b0000_1101_1101_1111_1111_1111_1111_1111;

        assert_eq!(
            LoadStoreCoprocessorMode::Unindexed,
            LoadStoreCoprocessorMode::from(unindexed)
        );

        assert_eq!(
            LoadStoreCoprocessorMode::ImmediatePostIndexed,
            LoadStoreCoprocessorMode::from(immediate_post_indexed)
        );

        assert_eq!(
            LoadStoreCoprocessorMode::ImmediatePreIndexed,
            LoadStoreCoprocessorMode::from(immediate_pre_indexed)
        );

        assert_eq!(
            LoadStoreCoprocessorMode::ImmediateOffset,
            LoadStoreCoprocessorMode::from(immediate_offset)
        );
    }
}
