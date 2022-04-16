use crate::{
    cpus::general::instruction::arm::{
        types::Register,
        BitState,
    },
    ram::Word,
};

use super::ArmOperand;

use std::convert::TryFrom;

pub fn get_ldc_stc_operand(value: Word) -> ArmOperand {
    let u = BitState::new(value, 23);
    let n = BitState::new(value, 22);
    let rn = Register::new(value, 16, 0b1111);
    let crd = Register::new(value, 12, 0b1111);
    let cp_num = u8::try_from((value >> 8) & 0b1111).unwrap();
    let immed8 = u8::try_from(value & 0b1111_1111).unwrap();

    ArmOperand::LDCandSTC {
        u,
        n,
        rn,
        crd,
        cp_num,
        immed8,
        mode: LoadStoreCoprocessorMode::from(value),
    }
}

pub fn get_mcr_mrc_operand(value: Word) -> ArmOperand {
    ArmOperand::MCRandMRC {
        opcode1: u8::try_from((value >> 21) & 0b111).unwrap(),
        crn: Register::new(value, 16, 0b1111),
        rd: Register::new(value, 12, 0b1111),
        cp_num: u8::try_from((value >> 8) & 0b1111).unwrap(),
        opcode2: u8::try_from((value >> 5) & 0b111).unwrap(),
        crm: Register::new(value, 0, 0b1111),
    }
}

pub fn get_mcrr_mrrc_operand(value: Word) -> ArmOperand {
    ArmOperand::MCRRandMRRC {
        rn: Register::new(value, 16, 0b1111),
        rd: Register::new(value, 12, 0b1111),
        cp_num: u8::try_from((value >> 8) & 0b1111).unwrap(),
        opcode: u8::try_from((value >> 4) & 0b1111).unwrap(),
        crm: Register::new(value, 0, 0b1111),
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
    use crate::cpus::general::instruction::arm::{
        operand::{
            load_store_coprocessor::{
                get_ldc_stc_operand,
                get_mcr_mrc_operand,
                LoadStoreCoprocessorMode, get_mcrr_mrrc_operand,
            },
            ArmOperand,
        },
        types::Register,
        BitState,
    };

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

    #[test]
    fn test_get_ldc_stc_operand() {
        let value = 0b0000_1101_1110_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmOperand::LDCandSTC {
                u: BitState::SET,
                n: BitState::SET,
                rn: Register::from(0b1111),
                crd: Register::from(0b1111),
                cp_num: 0b1111,
                immed8: 0b1111_1111,
                // it has its own tests
                mode: LoadStoreCoprocessorMode::from(value),
            },
            get_ldc_stc_operand(value)
        );
    }

    #[test]
    fn test_get_mcr_mrc_operand() {
        let value = 0b0000_1110_1110_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmOperand::MCRandMRC {
                opcode1: 0b111,
                crn: Register::from(0b1111),
                rd: Register::from(0b1111),
                cp_num: 0b1111,
                opcode2: 0b111,
                crm: Register::from(0b1111)
            },
            get_mcr_mrc_operand(value)
        );
    }

    #[test]
    fn test_get_mcrr_mrrc_operand() {
        let value = 0b0000_1100_0100_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmOperand::MCRRandMRRC {
                rn: Register::from(0b1111),
                rd: Register::from(0b111),
                cp_num: 0b1111,
                opcode: 0b1111,
                crm: Register::from(0b111),
            },
            get_mcrr_mrrc_operand(value)
        );
    }
}
