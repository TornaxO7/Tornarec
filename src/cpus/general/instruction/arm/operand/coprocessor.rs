use crate::{
    cpus::general::instruction::{
        arm::types::Register,
        types::BitState,
    },
    ram::Word,
};

use super::ArmOperand;

use std::convert::TryFrom;

pub fn get_cdp(value: Word) -> ArmOperand {
    let opcode1 = u8::try_from((value >> 20) & 0b1111).unwrap();
    let num = u8::try_from((value >> 8) & 0b1111).unwrap();
    let opcode2 = u8::try_from((value >> 5) & 0b111).unwrap();

    ArmOperand::CDP {
        opcode1,
        crn: Register::new(value, 16, 0b1111),
        crd: Register::new(value, 12, 0b1111),
        num,
        opcode2,
        crm: Register::new(value, 0, 0b1111),
    }
}

pub fn get_ldc_stc(value: Word) -> ArmOperand {
    ArmOperand::LDCandSTC {
        u: BitState::new(value, 23),
        n: BitState::new(value, 22),
        rn: Register::new(value, 16, 0b1111),
        crd: Register::new(value, 12, 0b1111),
        cp_num: u8::try_from((value >> 8) & 0b1111).unwrap(),
        immed8: u8::try_from(value & 0b1111_1111).unwrap(),
        // `p` and `w` are used here
        mode: LoadStoreCoprocessorMode::from(value),
    }
}

pub fn get_mcr_mrc(value: Word) -> ArmOperand {
    ArmOperand::MCRandMRC {
        opcode1: u8::try_from((value >> 21) & 0b111).unwrap(),
        crn: Register::new(value, 16, 0b1111),
        rd: Register::new(value, 12, 0b1111),
        cp_num: u8::try_from((value >> 8) & 0b1111).unwrap(),
        opcode2: u8::try_from((value >> 5) & 0b111).unwrap(),
        crm: Register::new(value, 0, 0b1111),
    }
}

pub fn get_mcrr_mrrc(value: Word) -> ArmOperand {
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
mod tests {

    use super::{
        get_cdp,
        get_ldc_stc,
        get_mcr_mrc,
        get_mcrr_mrrc,
        ArmOperand,
        BitState,
        LoadStoreCoprocessorMode,
        Register,
    };

    #[test]
    fn test_get_cdp() {
        let value = 0b0000_1110_1111_1111_1111_1111_1110_1111;

        let operand = get_cdp(value);
        let expected = ArmOperand::CDP {
            opcode1: u8::from(0b1111),
            crn: Register::from(0b1111),
            crd: Register::from(0b1111),
            num: 0b1111,
            opcode2: 0b111,
            crm: Register::from(0b1111),
        };
        assert_eq!(expected, operand, "{:#?}, {:#?}", expected, operand);
    }

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

        let operand = get_ldc_stc(value);
        let expected = ArmOperand::LDCandSTC {
            u: BitState::SET,
            n: BitState::SET,
            rn: Register::from(0b1111),
            crd: Register::from(0b1111),
            cp_num: 0b1111,
            immed8: 0b1111_1111,
            // it has its own tests
            mode: LoadStoreCoprocessorMode::from(value),
        };

        assert_eq!(expected, operand, "{:#?}, {:#?}", expected, operand);
    }

    #[test]
    fn test_get_mcr_mrc_operand() {
        let value = 0b0000_1110_1110_1111_1111_1111_1111_1111;

        let operand = get_mcr_mrc(value);
        let expected = ArmOperand::MCRandMRC {
            opcode1: 0b111,
            crn: Register::from(0b1111),
            rd: Register::from(0b1111),
            cp_num: 0b1111,
            opcode2: 0b111,
            crm: Register::from(0b1111),
        };

        assert_eq!(expected, operand, "{:#?} {:#?}", expected, operand);
    }

    #[test]
    fn test_get_mcrr_mrrc_operand() {
        let value = 0b0000_1100_0100_1111_1111_1111_1111_1111;

        let operand = get_mcrr_mrrc(value);
        let expected = ArmOperand::MCRRandMRRC {
            rn: Register::from(0b1111),
            rd: Register::from(0b1111),
            cp_num: 0b1111,
            opcode: 0b1111,
            crm: Register::from(0b1111),
        };

        assert_eq!(expected, operand, "{:#?}, {:#?}", expected, operand);
    }
}
