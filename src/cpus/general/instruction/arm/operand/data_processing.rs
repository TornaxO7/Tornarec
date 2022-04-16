use std::convert::TryFrom;

use crate::{
    cpus::general::instruction::arm::{
        BitState,
        Register,
    },
    ram::Word,
};

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
    let s = BitState::from(((value >> 20) & 0b1) != 0);
    let rn = Register::try_from((value >> 16) & 0b1111).unwrap();
    let rd = Register::try_from((value >> 12) & 0b1111).unwrap();

    let bit25 = BitState::from(((value >> 25) & 0b1) != 0);
    let bit7 = BitState::from(((value >> 7) & 0b1) != 0);
    let bit4 = BitState::from(((value >> 4) & 0b1) != 0);
    let shifter_operand = if bit25 {
        ShifterOperand::get_immediate(value)
    } else if !bit4 {
        ShifterOperand::get_immediate_shift(value)
    } else if !bit7 && bit4 {
        ShifterOperand::get_register_shift(value)
    } else {
        unreachable!()
    };

    ArmOperand::DataProcessing {
        s,
        rn,
        rd,
        shifter_operand,
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
            rm: Register::try_from(value & 0b1111).unwrap(),
        }
    }

    pub fn get_register_shift(value: Word) -> Self {
        Self::RegisterShift {
            rs: Register::try_from((value >> 8) & 0b1111).unwrap(),
            shift: u8::try_from((value >> 5) & 0b11).unwrap(),
            rm: Register::try_from(value & 0b1111).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpus::general::instruction::arm::{
        operand::{
            data_processing::{
                get_operand,
                ShifterOperand,
            },
            ArmOperand,
        },
        BitState,
        Register,
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
        let value = 0b0000_0001_1111_1111_1111_1111_1110_11111;

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
                s: BitState::from(true),
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
                s: BitState::from(true),
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
                s: BitState::from(true),
                rn: Register::from(0b1111),
                rd: Register::from(0b1111),
                shifter_operand: ShifterOperand::get_register_shift(value),
            },
            get_operand(value),
        );
    }
}
