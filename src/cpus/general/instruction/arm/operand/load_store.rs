use crate::{
    cpus::general::instruction::arm::{
        types::{
            sbz,
            Register,
            RegisterList,
        },
        BitState,
    },
    ram::Word,
};

use std::convert::TryFrom;

use super::ArmOperand;

/// A3.11.2 (page 130) Load and store word or unsigned byte
pub fn get_word_or_unsigned_byte(value: Word) -> ArmOperand {
    ArmOperand::LoadStore {
        rn: Register::new(value, 16, 0b1111),
        load_store_type: LoadStoreType::get_word_or_unsigned_byte(value),
    }
}

/// A3.11.3 (page 131) Load and store halfword or doubleword, and load signed byte
pub fn get_misc(value: Word) -> ArmOperand {
    ArmOperand::LoadStore {
        rn: Register::new(value, 16, 0b1111),
        load_store_type: LoadStoreType::get_halfword_doubleword_or_signed(value),
    }
}

/// A3.12 (page 134) Load and Store Multiple
pub fn get_multiple(value: Word) -> ArmOperand {
    ArmOperand::LoadStore {
        rn: Register::new(value, 16, 0b1111),
        load_store_type: LoadStoreType::get_multiple(value),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadStoreType {
    /// The B flag is already in the name of the instruction
    WordOrUnsignedByte {
        u: BitState,
        rd: Register,
        address_mode: AddressingMode2,
        index_type: LoadStoreIndexType,
    },
    HalfwordOrDoublewordAndLoadSignedByte {
        rd: Register,
        address_mode: AddressingMode3,
        s: BitState,
        h: BitState,
        index_type: LoadStoreIndexType,
    },
    Multiple {
        s: BitState,
        w: BitState,
        reg_list: RegisterList,
        address_mode: AddressingMode4,
    },
}

impl LoadStoreType {
    pub fn get_word_or_unsigned_byte(value: Word) -> Self {
        Self::WordOrUnsignedByte {
            u: BitState::new(value, 23),
            rd: Register::new(value, 12, 0b1111),
            address_mode: AddressingMode2::from(value),
            index_type: LoadStoreIndexType::from(value),
        }
    }

    pub fn get_halfword_doubleword_or_signed(value: Word) -> Self {
        Self::HalfwordOrDoublewordAndLoadSignedByte {
            rd: Register::new(value, 12, 0b1111),
            address_mode: AddressingMode3::from(value),
            s: BitState::new(value, 6),
            h: BitState::new(value, 5),
            index_type: LoadStoreIndexType::from(value),
        }
    }

    pub fn get_multiple(value: Word) -> Self {
        Self::Multiple {
            s: BitState::new(value, 22),
            w: BitState::new(value, 21),
            reg_list: RegisterList::new(value, 0, 0b1111_1111_1111_1111),
            address_mode: AddressingMode4::from(value),
        }
    }
}

impl From<Word> for LoadStoreType {
    fn from(value: Word) -> Self {
        let bit27_26 = (value >> 26) & 0b11;
        match bit27_26 {
            0b01 => LoadStoreType::get_word_or_unsigned_byte(value),
            0b00 => LoadStoreType::get_halfword_doubleword_or_signed(value),
            0b10 => LoadStoreType::get_multiple(value),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadStoreIndexType {
    Offset,
    PreIndexed,
    PostIndexed,
}

impl From<Word> for LoadStoreIndexType {
    fn from(value: Word) -> Self {
        let p = BitState::new(value, 24);
        let w = BitState::new(value, 21);

        match (p, w) {
            (BitState::SET, BitState::UNSET) => Self::Offset,
            (BitState::SET, BitState::SET) => Self::PreIndexed,
            (BitState::UNSET, BitState::UNSET) => Self::PostIndexed,
            _ => unreachable!(),
        }
    }
}

/// Page 458. "Register offset/index" isn't implemented since it's literally the
/// same as "Scaled register offset/index", just with `shift_imm = 0 = shift`...
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AddressingMode2 {
    Immediate(u16),
    Register {
        shift_imm: u8,
        shift: u8,
        rm: Register,
    },
}

impl AddressingMode2 {
    pub fn get_immediate(value: Word) -> Self {
        let immed12 = u16::try_from(value & 0b1111_1111_1111).unwrap();
        Self::Immediate(immed12)
    }

    pub fn get_register(value: Word) -> Self {
        Self::Register {
            shift_imm: u8::try_from((value >> 7) & 0b1_1111).unwrap(),
            shift: u8::try_from((value >> 5) & 0b11).unwrap(),
            rm: Register::new(value, 0, 0b1111),
        }
    }
}

impl From<Word> for AddressingMode2 {
    fn from(value: Word) -> Self {
        let bit25 = BitState::new(value, 25);
        match bit25 {
            BitState::SET => Self::get_register(value),
            BitState::UNSET => Self::get_immediate(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AddressingMode3 {
    Immediate { immedh: u8, immedl: u8 },
    Register(Register),
}

impl AddressingMode3 {
    pub fn get_immediate(value: Word) -> Self {
        Self::Immediate {
            immedh: u8::try_from((value >> 8) & 0b1111).unwrap(),
            immedl: u8::try_from(value & 0b1111).unwrap(),
        }
    }

    pub fn get_register(value: Word) -> Self {
        let register = Register::new(value, 0, 0b1111);

        sbz(value, 8, 0b1111);
        Self::Register(register)
    }
}

impl From<Word> for AddressingMode3 {
    fn from(value: Word) -> Self {
        let bit22 = BitState::new(value, 22);
        match bit22 {
            BitState::SET => Self::get_immediate(value),
            BitState::UNSET => Self::get_register(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AddressingMode4 {
    IncrementAfter,
    IncrementBefore,
    DecrementAfter,
    DecrementBefore,
}

impl From<Word> for AddressingMode4 {
    fn from(value: Word) -> Self {
        let p = BitState::new(value, 24);
        let u = BitState::new(value, 23);

        match (p, u) {
            (BitState::UNSET, BitState::UNSET) => Self::DecrementAfter,
            (BitState::UNSET, BitState::SET) => Self::IncrementAfter,
            (BitState::SET, BitState::UNSET) => Self::DecrementBefore,
            (BitState::SET, BitState::SET) => Self::IncrementBefore,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AddressingMode2,
        AddressingMode3,
        LoadStoreIndexType,
        Register,
    };

    #[test]
    fn test_addressing_mode2_immediate() {
        let value = 0b0000_0101_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            AddressingMode2::Immediate(0b1111_1111_1111),
            AddressingMode2::get_immediate(value)
        );
    }

    #[test]
    fn test_addressing_mode2_register() {
        let value = 0b0000_0111_1111_1111_1111_1111_1110_1111;

        assert_eq!(
            AddressingMode2::Register {
                shift_imm: 0b1_1111,
                shift: 0b11,
                rm: Register::from(0b1111),
            },
            AddressingMode2::get_register(value)
        );
    }

    #[test]
    fn test_addressing_mode3_get_immediate() {
        let value = 0b0000_0001_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            AddressingMode3::Immediate {
                immedh: 0b1111,
                immedl: 0b1111,
            },
            AddressingMode3::from(value)
        );
    }

    #[test]
    fn test_addressing_mode3_get_register() {
        let value = 0b0000_0001_1011_1111_1111_0000_1111_1111;

        assert_eq!(
            AddressingMode3::Register(Register::from(0b1111)),
            AddressingMode3::get_register(value)
        );
    }

    #[test]
    #[should_panic]
    fn test_addressing_mode3_get_register_sbz() {
        let value = 0b0000_0001_1011_1111_1111_1111_1111_1111;
        AddressingMode3::get_register(value);
    }

    #[test]
    fn test_load_store_index_type_offset() {
        let value = 0b0000_0101_1101_1111_1111_1111_1111_1111;
        assert_eq!(LoadStoreIndexType::Offset, LoadStoreIndexType::from(value));
    }

    #[test]
    fn test_load_store_index_type_pre_indexed() {
        let value = 0b0000_0101_1111_1111_1111_1111_1111_1111;
        assert_eq!(
            LoadStoreIndexType::PreIndexed,
            LoadStoreIndexType::from(value)
        );
    }

    #[test]
    fn test_load_store_index_type_post_indexed() {
        let value = 0b0000_0100_1101_1111_1111_1111_1111_1111;
        assert_eq!(
            LoadStoreIndexType::PostIndexed,
            LoadStoreIndexType::from(value)
        );
    }
}
