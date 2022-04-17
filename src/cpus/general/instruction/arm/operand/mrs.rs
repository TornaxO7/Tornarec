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

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
    sbo(value, 16, 0b1111);
    sbz(value, 0, 0b1111_1111_1111);

    ArmOperand::MRS {
        r: BitState::new(value, 22),
        rd: Register::new(value, 12, 0b1111),
    }
}

#[cfg(test)]
mod tests {

    use super::{
        get_operand,
        ArmOperand,
        BitState,
        Register,
    };

    #[test]
    fn test_get_operand() {
        let value = 0b0000_0001_0100_1111_1111_0000_0000_0000;

        assert_eq!(
            ArmOperand::MRS {
                r: BitState::SET,
                rd: Register::from(0b1111),
            },
            get_operand(value),
        );
    }

    #[test]
    #[should_panic]
    fn test_get_operand_sbo() {
        let value = 0b0000_0001_0100_0000_1111_0000_0000_0000;
        get_operand(value);
    }

    #[test]
    #[should_panic]
    fn test_get_operand_sbz() {
        let value = 0b0000_0001_0100_1111_1111_1111_1111_1111;
        get_operand(value);
    }
}
