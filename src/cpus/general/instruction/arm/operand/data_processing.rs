use std::convert::TryFrom;

use crate::{
    cpus::general::instruction::arm::{
        types::Register,
        BitState,
    },
    ram::Word,
};

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
    ArmOperand::DataProcessing {
        s: BitState::new(value, 20),
        rn: Register::new(value, 16, 0b1111),
        rd: Register::new(value, 12, 0b1111),
        shifter_operand: ShifterOperand::from(value),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShifterOperand {
    Immediate {
        rotate_imm: u8,
        immed8: u8,
    },
    ImmediateShift {
        shift_imm: u8,
        shift: u8,
        rm: Register,
    },
    RegisterShift {
        rs: Register,
        shift: u8,
        rm: Register,
    },
}

impl ShifterOperand {
    pub fn get_immediate(value: Word) -> Self {
        Self::Immediate {
            rotate_imm: u8::try_from((value >> 8) & 0b1111).unwrap(),
            immed8: u8::try_from(value & 0b1111_1111).unwrap(),
        }
    }

    pub fn get_immediate_shift(value: Word) -> Self {
        Self::ImmediateShift {
            shift_imm: u8::try_from((value >> 7) & 0b1_1111).unwrap(),
            shift: u8::try_from((value >> 5) & 0b11).unwrap(),
            rm: Register::new(value, 0, 0b1111),
        }
    }

    pub fn get_register_shift(value: Word) -> Self {
        Self::RegisterShift {
            rs: Register::new(value, 8, 0b1111),
            shift: u8::try_from((value >> 5) & 0b11).unwrap(),
            rm: Register::new(value, 0, 0b1111),
        }
    }
}

impl From<Word> for ShifterOperand {
    fn from(value: Word) -> Self {
        let bit25 = BitState::new(value, 25);
        let bit7 = BitState::new(value, 7);
        let bit4 = BitState::new(value, 4);

        if *bit25 {
            ShifterOperand::get_immediate(value)
        } else if !bit4 {
            ShifterOperand::get_immediate_shift(value)
        } else if !bit7 && *bit4 {
            ShifterOperand::get_register_shift(value)
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{
        ShifterOperand,
        ArmOperand,
        Register,
        get_operand,
        BitState
    };

    #[test]
    fn shifter_operand_immediate() {
        let value = 0b0000_0011_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            ShifterOperand::Immediate {
                rotate_imm: 0b1111,
                immed8: 0b1111_1111
            },
            ShifterOperand::get_immediate(value)
        );
    }

    #[test]
    fn shifter_operand_immediate_shift() {
        let value = 0b0000_0001_1111_1111_1111_1111_1110_1111;

        assert_eq!(
            ShifterOperand::ImmediateShift {
                shift_imm: 0b1_1111,
                shift: 0b11,
                rm: Register::from(0b1111),
            },
            ShifterOperand::get_immediate_shift(value)
        );
    }

    #[test]
    fn test_data_processing_immediate() {
        let value = 0b0000_0011_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmOperand::DataProcessing {
                s: BitState::SET,
                rn: Register::from(0b1111),
                rd: Register::from(0b1111),
                shifter_operand: ShifterOperand::get_immediate(value),
            },
            get_operand(value)
        );
    }

    #[test]
    fn test_data_processing_immediate_shift() {
        let value = 0b0000_0001_1111_1111_1111_1111_1110_1111;

        assert_eq!(
            ArmOperand::DataProcessing {
                s: BitState::SET,
                rn: Register::from(0b1111),
                rd: Register::from(0b1111),
                shifter_operand: ShifterOperand::get_immediate_shift(value),
            },
            get_operand(value)
        );
    }

    #[test]
    fn test_data_processing_register_shift() {
        let value = 0b0000_0001_1111_1111_1111_1111_0111_1111;

        assert_eq!(
            ArmOperand::DataProcessing {
                s: BitState::SET,
                rn: Register::from(0b1111),
                rd: Register::from(0b1111),
                shifter_operand: ShifterOperand::get_register_shift(value),
            },
            get_operand(value),
        );
    }
}
