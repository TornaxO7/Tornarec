use crate::{
    cpus::general::instruction::arm::{
        BitState,
        Register,
    },
    ram::Word,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode1Offset {
    Immediate {
        rotate_imm: u8,
        immed8: u8,
    },
    ImmediateShift {
        shift_imm: u8,
        shift: u8,
        rm: u8,
    },
    RegisterShift {
        rs: Register,
        shift: u8,
        rm: Register,
    },
}

impl AddressingMode1Offset {
    pub fn get_immediate(value: Word) -> Self {
        let rotate_imm = u8::try_from((value >> 8) & 0b1111).unwrap();
        let immed8 = u8::try_from(value & 0b1111_1111).unwrap();

        Self::Immediate { rotate_imm, immed8 }
    }

    pub fn get_immediate_shift(value: Word) -> Self {
        let shift_imm = u8::try_from((value >> 7) & 0b11111).unwrap();
        let shift = u8::try_from((value >> 5) & 0b11).unwrap();
        let rm = u8::try_from(value & 0b1111).unwrap();

        Self::ImmediateShift {
            shift_imm,
            shift,
            rm,
        }
    }

    pub fn get_register_shift(value: Word) -> Self {
        let rs = u8::try_from((value >> 8) & 0b1111).unwrap();
        let shift = u8::try_from((value >> 5) & 0b11).unwrap();
        let rm = u8::try_from(value & 0b1111).unwrap();

        Self::RegisterShift { rs, shift, rm }
    }

    pub fn is_immediate(value: Word) -> bool {
        let bit25 = BitState::from(((value >> 25) & 0b1) != 0);

        bit25
    }

    pub fn is_immediate_shift(value: Word) -> bool {
        let bit4 = BitState::from(((value >> 4) & 0b1) != 0);

        !AddressingMode1Offset::is_immediate(value) && !bit4
    }

    pub fn is_register_shift(value: Word) -> bool {
        let bit7 = BitState::from(((value >> 7) & 0b1) != 0);

        !AddressingMode1Offset::is_immediate(value)
            && !bit7
            && AddressingMode1Offset::is_immediate_shift(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode2Offset {
    ImmediateOffset(u16),
    RegisterOffset(Register),
    ScaledRegisterOffset {
        shift_imm: u8,
        shift: u8,
        rm: Register,
    },
}

impl AddressingMode2Offset {
    pub fn get_immediate_offset(value: Word) -> Self {
        let immed_offset = u16::try_from(value & 0b1111_1111_1111).unwrap();
        Self::ImmediateOffset(immed_offset)
    }

    pub fn get_register_offset(value: Word) -> Self {
        let rm = Register::try_from(value & 0b1111).unwrap();

        Self::RegisterOffset(rm)
    }

    pub fn get_scaled_register_offset(value: Word) -> Self {
        let shift_imm = u8::try_from((value >> 7) & 0b1111).unwrap();
        let shift = u8::try_from((value >> 5) & 0b11).unwrap();
        let rm = Register::try_from(value & 0b1111).unwrap();

        Self::ScaledRegisterOffset {
            shift_imm,
            shift,
            rm,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode3Offset {
    Immediate {
        immed_h: u8,
        s: BitState,
        h: BitState,
        immed_l: u8,
    },
    Register {
        s: BitState,
        h: BitState,
        rm: Register,
    },
}

impl AddressingMode3Offset {
    pub fn get_immediate_offset(value: Word) -> Self {
        let immed_h = u8::try_from((value >> 8) & 0b1111).unwrap();
        let s = BitState::from(((value >> 6) & 1) != 0);
        let h = BitState::from(((value >> 5) & 1) != 0);
        let immed_l = u8::try_from(value & 0b1111).unwrap();

        Self::Immediate {
            immed_h,
            s,
            h,
            immed_l,
        }
    }

    pub fn get_register_offset(value: Word) -> Self {
        let s = BitState::from(((value >> 6) & 1) != 0);
        let h = BitState::from(((value >> 5) & 1) != 0);
        let rm = Register::try_from(value & 0b1111).unwrap();

        Self::Register { s, h, rm }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode4Offset {
    IncrementAfter,
    IncrementBefore,
    DecrementAfter,
    DecrementBefore,
}

impl From<Word> for AddressingMode4Offset {
    fn from(value: Word) -> Self {
        let p_flag = (value >> 24) & 0b1;
        let u_flag = (value >> 23) & 0b1;

        match (p_flag, u_flag) {
            (0, 1) => Self::IncrementAfter,
            (1, 1) => Self::IncrementBefore,
            (0, 0) => Self::DecrementAfter,
            (1, 0) => Self::DecrementBefore,
        }
    }
}

/// Coprocessor addressing mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode5Offset {
    ImmediateOffset,
    ImmediatePreIndexed,
    ImmediatePostIndexed,
    Unindexed,
}

impl From<Word> for AddressingMode5Offset {
    fn from(value: Word) -> Self {
        let p_flag = (value >> 24) & 0b1;
        let w_flag = (value >> 21) & 0b1;

        match (p_flag, w_flag) {
            (1, 0) => Self::ImmediateOffset,
            (1, 1) => Self::ImmediatePreIndexed,
            (0, 1) => Self::ImmediatePostIndexed,
            (0, 0) => Self::Unindexed,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpus::general::instruction::arm::{
        encoding_fields::{
            AddressingMode2Offset,
            AddressingMode3Offset,
            AddressingMode4Offset,
            AddressingMode5Offset,
        },
        Register,
    };

    use super::AddressingMode1Offset;

    #[test]
    fn addressing_mode_1_offset_immediate() {
        let value = 0b0000_0000_0000_0000_0000_1111_1111_1111;

        assert_eq!(
            AddressingMode1Offset::Immediate {
                rotate_imm: 0b1111_1111,
                immed8: 0b1111
            },
            AddressingMode1Offset::get_immediate(value)
        );
    }

    #[test]
    fn addressing_mode_1_offset_immediate_shift() {
        let value = 0b0000_0000_0000_0000_0000_1111_110_1111;

        assert_eq!(
            AddressingMode1Offset::ImmediateShift {
                shift_imm: 0b1111,
                shift: 0b11,
                rm: 0b1111
            },
            AddressingMode1Offset::get_immediate_shift(value)
        );
    }

    #[test]
    fn addressing_mode_1_offset_register_shift() {
        let value = 0b0000_0000_0000_0000_0000_1111_0_11_1_1111;

        assert_eq!(
            AddressingMode1Offset::RegisterShift {
                rs: 0b1111,
                shift: 0b11,
                rm: 0b1111,
            },
            AddressingMode1Offset::get_register_shift(value)
        );
    }

    #[test]
    fn addresing_mode_2_immediate_offset() {
        let value = 0b0000_0000_0000_0000_0000_1111_1111_1111;

        assert_eq!(
            AddressingMode2Offset::get_immediate_offset(0b1111_1111_1111),
            AddressingMode2Offset::ImmediateOffset(0b1111_1111_1111)
        );
    }

    #[test]
    fn addressing_mode_2_register_offset() {
        let value = 0b0000_0000_0000_0000_0000_0000_0000_1111;

        assert_eq!(
            AddressingMode2Offset::get_register_offset(value),
            AddressingMode2Offset::RegisterOffset(Register::from(0b1111))
        );
    }

    #[test]
    fn addressing_mode_2_scaled_register_offset() {
        let value = 0b0000_0000_0000_0000_0000_1111_1110_1111;

        assert_eq!(
            AddressingMode2Offset::get_scaled_register_offset(value),
            AddressingMode2Offset::ScaledRegisterOffset {
                shift_imm: 0b11111,
                shift: 0b11,
                rm: Register::from(0b1111)
            }
        );
    }

    #[test]
    fn addressing_mode_3_immediate() {
        let value = 0b0000_0000_0000_0000_0000_1111_1_11_1_1111;

        assert_eq!(
            AddressingMode3Offset::get_immediate_offset(value),
            AddressingMode3Offset::Immediate {
                immed_h: 0b1111,
                s: true,
                h: true,
                immed_l: 0b1111
            }
        );
    }

    #[test]
    fn addressing_mode_3_register() {
        let value = 0b0000_0000_0000_0000_0000_0000_1_11_1_1111;

        assert_eq!(
            AddressingMode3Offset::get_register_offset(value),
            AddressingMode3Offset::Register {
                s: true,
                h: true,
                rm: Register::from(0b1111),
            }
        );
    }

    #[test]
    fn addressing_mode_4_increment_after() {
        let value = 0b0000_1000_1000_0000_0000_0000_0000_0000;

        assert_eq!(
            AddressingMode4Offset::from(value),
            AddressingMode4Offset::IncrementAfter
        );
    }

    #[test]
    fn addressing_mode_4_increment_before() {
        let value = 0b0000_1001_1000_0000_0000_0000_0000_0000;

        assert_eq!(
            AddressingMode4Offset::from(value),
            AddressingMode4Offset::IncrementBefore
        );
    }

    #[test]
    fn addressing_mode_4_decrement_after() {
        let value = 0b0000_1000_0000_0000_0000_0000_0000_0000;

        assert_eq!(
            AddressingMode4Offset::from(value),
            AddressingMode4Offset::DecrementAfter
        );
    }

    #[test]
    fn addressing_mode_4_decrement_before() {
        let value = 0b0000_1001_0000_0000_0000_0000_0000_0000;

        assert_eq!(
            AddressingMode4Offset::from(value),
            AddressingMode4Offset::DecrementBefore
        );
    }

    #[test]
    fn addressing_mode_5_immediate_offset() {
        let value = 0b0000_1101_0000_0000_0000_0000_0000_0000;

        assert_eq!(
            AddressingMode5Offset::from(value),
            AddressingMode5Offset::ImmediateOffset
        );
    }

    #[test]
    fn addressing_mode_5_immediate_pre_indexed() {
        let value = 0b0000_1101_0010_0000_0000_0000_0000_0000;

        assert_eq!(
            AddressingMode5Offset::from(value),
            AddressingMode5Offset::ImmediatePreIndexed
        );
    }

    #[test]
    fn addressing_mode_5_immediate_post_indexed() {
        let value = 0b0000_1100_0010_0000_0000_0000_0000_0000;

        assert_eq!(
            AddressingMode5Offset::from(value),
            AddressingMode5Offset::ImmediatePostIndexed
        );
    }

    #[test]
    fn addressing_mode_5_unindexed() {
        let value = 0b0000_1100_0000_0000_0000_0000_0000_0000;

        assert_eq!(
            AddressingMode5Offset::from(value),
            AddressingMode5Offset::Unindexed
        );
    }
}
