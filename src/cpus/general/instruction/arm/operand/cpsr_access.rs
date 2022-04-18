use crate::{
    cpus::general::instruction::arm::{
        types::{
            sbo,
            sbz,
            Register,
        },
        BitState,
    },
    ram::Word,
};

use super::{
    data_processing::ShifterOperand,
    ArmOperand,
};

use std::convert::TryFrom;

pub fn get_mrs(value: Word) -> ArmOperand {
    sbo(value, 16, 0b1111);
    sbz(value, 0, 0b1111_1111_1111);

    ArmOperand::MRS {
        r: BitState::new(value, 22),
        rd: Register::new(value, 12, 0b1111),
    }
}

pub fn get_msr(value: Word) -> ArmOperand {
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

    use super::{
        get_mrs,
        get_msr,
        ArmOperand,
        BitState,
        Register,
        ShifterOperand,
    };

    #[test]
    fn test_get_mrs() {
        let value = 0b0000_0001_0100_1111_1111_0000_0000_0000;

        assert_eq!(
            ArmOperand::MRS {
                r: BitState::SET,
                rd: Register::from(0b1111),
            },
            get_mrs(value),
        );
    }

    #[test]
    #[should_panic]
    fn test_get_mrs_sbo() {
        let value = 0b0000_0001_0100_0000_1111_0000_0000_0000;
        get_mrs(value);
    }

    #[test]
    #[should_panic]
    fn test_get_mrs_sbz() {
        let value = 0b0000_0001_0100_1111_1111_1111_1111_1111;
        get_mrs(value);
    }

    #[test]
    fn test_get_msr_immediate() {
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
            get_msr(value)
        );
    }

    #[test]
    fn test_get_msr_register() {
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
            get_msr(value)
        );
    }

    #[test]
    #[should_panic]
    fn test_get_msr_immediate_sbo() {
        let value = 0b0000_0011_0110_1111_0000_1111_1111_1111;
        get_msr(value);
    }

    #[test]
    #[should_panic]
    fn test_get_msr_register_sbo() {
        let value = 0b0000_0001_0110_1111_0000_0000_0000_1111;
        get_msr(value);
    }

    #[test]
    #[should_panic]
    fn test_get_msr_register_sbz() {
        let value = 0b0000_0001_0110_1111_1111_1111_0000_1111;
        get_msr(value);
    }
}
