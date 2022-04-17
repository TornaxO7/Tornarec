use crate::{
    cpus::general::instruction::arm::{
        types::{
            sbo,
            sbz,
        },
        BitState,
    },
    ram::Word,
};

use std::convert::TryFrom;

use super::{
    data_processing::ShifterOperand,
    ArmOperand,
};

pub fn get_operand(value: Word) -> ArmOperand {
    let shifter_operand = {
        let bit25 = BitState::new(value, 25);
        match bit25 {
            BitState::SET => ShifterOperand::get_immediate(value),
            BitState::UNSET => ShifterOperand::get_register_shift(value),
        }
    };

    sbo(value, 12, 0b1111);
    if let ShifterOperand::RegisterShift { .. } = shifter_operand {
        sbz(value, 4, 0b1111_1111);
    }

    ArmOperand::MSR {
        r: BitState::new(value, 22),
        field_mask: u8::try_from((value >> 16) & 0b1111).unwrap(),
        shifter_operand,
    }
}

#[cfg(test)]
mod tests {

    use crate::cpus::general::instruction::arm::types::Register;

    use super::{
        get_operand,
        ArmOperand,
        BitState,
        ShifterOperand,
    };

    #[test]
    fn test_get_operand_immediate() {
        let value = 0b0000_0011_0110_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmOperand::MSR {
                r: BitState::SET,
                field_mask: 0b1111,
                shifter_operand: ShifterOperand::Immediate {
                    rotate_imm: 0b1111,
                    immed8: 0b1111_1111,
                }
            },
            get_operand(value)
        );
    }

    #[test]
    fn test_get_operand_register() {
        let value = 0b0000_0001_0110_1111_1111_0000_0000_1111;

        assert_eq!(
            ArmOperand::MSR {
                r: BitState::SET,
                field_mask: 0b1111,
                shifter_operand: ShifterOperand::RegisterShift {
                    rs: Register::from(0b0000),
                    shift: 0,
                    rm: Register::from(0b1111),
                }
            },
            get_operand(value)
        );
    }

    #[test]
    #[should_panic]
    fn test_get_operand_immediate_sbo() {
        let value = 0b0000_0011_0110_1111_0000_1111_1111_1111;
        get_operand(value);
    }

    #[test]
    #[should_panic]
    fn test_get_operand_register_sbo() {
        let value = 0b0000_0001_0110_1111_0000_0000_0000_1111;
        get_operand(value);
    }

    #[test]
    #[should_panic]
    fn test_get_operand_register_sbz() {
        let value = 0b0000_0001_0110_1111_1111_1111_0000_1111;
        get_operand(value);
    }
}
