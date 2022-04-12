use crate::{
    cpus::general::instruction::arm::{
        BitState,
        Register,
    },
    ram::Word,
};

use std::convert::TryFrom;

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
    pub fn get_immediate(word: &Word) -> Self {
        let rotate_imm = u8::try_from((word >> 8) & 0b1111).unwrap();
        let immed8 = u8::try_from(word & 0b1111_1111).unwrap();

        Self::Immediate { rotate_imm, immed8 }
    }

    pub fn get_immediate_shift(word: &Word) -> Self {
        let shift_imm = u8::try_from((word >> 7) & 0b11111).unwrap();
        let shift = u8::try_from((word >> 5) & 0b11).unwrap();
        let rm = u8::try_from(word & 0b1111).unwrap();

        Self::ImmediateShift {
            shift_imm,
            shift,
            rm,
        }
    }

    pub fn get_register_shift(word: &Word) -> Self {
        let rs = u8::try_from((word >> 8) & 0b1111).unwrap();
        let shift = u8::try_from((word >> 5) & 0b11).unwrap();
        let rm = u8::try_from(word & 0b1111).unwrap();

        Self::RegisterShift { rs, shift, rm }
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
    pub fn get_immediate_offset(word: &Word) -> Self {
        let immed_offset = u16::try_from(word & 0b1111_1111_1111).unwrap();
        Self::ImmediateOffset(immed_offset)
    }

    pub fn get_register_offset(word: &Word) -> Self {
        let rm = Register::try_from(word & 0b1111).unwrap();

        Self::RegisterOffset(rm)
    }

    pub fn get_scaled_register_offset(word: &Word) -> Self {
        let shift_imm = u8::try_from((word >> 7) & 0b1111).unwrap();
        let shift = u8::try_from((word >> 5) & 0b11).unwrap();
        let rm = Register::try_from(word & 0b1111).unwrap();

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
    pub fn get_immediate_offset(word: &Word) -> Self {
        let immed_h = u8::try_from((word >> 8) & 0b1111).unwrap();
        let s = BitState::from(((word >> 6) & 1) != 0);
        let h = BitState::from(((word >> 5) & 1) != 0);
        let immed_l = u8::try_from(word & 0b1111).unwrap();

        Self::Immediate {
            immed_h,
            s,
            h,
            immed_l,
        }
    }

    pub fn get_register_offset(word: &Word) -> Self {
        let s = BitState::from(((word >> 6) & 1) != 0);
        let h = BitState::from(((word >> 5) & 1) != 0);
        let rm = Register::try_from(word & 0b1111).unwrap();

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

/// Coprocessor addressing mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode5Offset {
    ImmediateOffset,
    ImmediatePreIndexed,
    ImmediatePostIndexed,
    Unindexed,
}

#[cfg(test)]
mod tests {
    use crate::cpus::general::instruction::arm::{
        encoding_fields::{
            AddressingMode2Offset,
            AddressingMode3Offset,
        },
        Register,
    };

    use super::AddressingMode1Offset;

    #[test]
    fn addressing_mode_1_offset_immediate() {
        let word = 0b0000_0000_0000_0000_0000_1111_1111_1111;

        assert_eq!(
            AddressingMode1Offset::Immediate {
                rotate_imm: 0b1111_1111,
                immed8: 0b1111
            },
            AddressingMode1Offset::get_immediate(word)
        );
    }

    #[test]
    fn addressing_mode_1_offset_immediate_shift() {
        let word = 0b0000_0000_0000_0000_0000_1111_110_1111;

        assert_eq!(
            AddressingMode1Offset::ImmediateShift {
                shift_imm: 0b1111,
                shift: 0b11,
                rm: 0b1111
            },
            AddressingMode1Offset::get_immediate_shift(word)
        );
    }

    #[test]
    fn addressing_mode_1_offset_register_shift() {
        let word = 0b0000_0000_0000_0000_0000_1111_0_11_1_1111;

        assert_eq!(
            AddressingMode1Offset::RegisterShift {
                rs: 0b1111,
                shift: 0b11,
                rm: 0b1111,
            },
            AddressingMode1Offset::get_register_shift(word)
        );
    }

    #[test]
    fn addresing_mode_2_immediate_offset() {
        let word = 0b0000_0000_0000_0000_0000_1111_1111_1111;

        assert_eq!(
            AddressingMode2Offset::get_immediate_offset(0b1111_1111_1111),
            AddressingMode2Offset::ImmediateOffset(0b1111_1111_1111)
        );
    }

    #[test]
    fn addressing_mode_2_register_offset() {
        let word = 0b0000_0000_0000_0000_0000_0000_0000_1111;

        assert_eq!(
            AddressingMode2Offset::get_register_offset(word),
            AddressingMode2Offset::RegisterOffset(Register::from(0b1111))
        );
    }

    #[test]
    fn addressing_mode_2_scaled_register_offset() {
        let word = 0b0000_0000_0000_0000_0000_1111_1110_1111;

        assert_eq!(
            AddressingMode2Offset::get_scaled_register_offset(word),
            AddressingMode2Offset::ScaledRegisterOffset {
                shift_imm: 0b11111,
                shift: 0b11,
                rm: Register::from(0b1111)
            }
        );
    }

    #[test]
    fn addressing_mode_3_immediate() {
        let word = 0b0000_0000_0000_0000_0000_1111_1_11_1_1111;

        assert_eq!(
            AddressingMode3Offset::get_immediate_offset(word),
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
        let word = 0b0000_0000_0000_0000_0000_0000_1_11_1_1111;

        assert_eq!(
            AddressingMode3Offset::get_register_offset(word),
            AddressingMode3Offset::Register {
                s: true,
                h: true,
                rm: Register::from(0b1111),
            }
        );
    }
}
