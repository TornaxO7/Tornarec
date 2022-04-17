use crate::{
    cpus::general::instruction::arm::{
        types::{
            sbz,
            Register,
        },
        BitState,
    },
    ram::Word,
};

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
    sbz(value, 12, 0b1111);
    ArmOperand::MUL {
        s: BitState::new(value, 20),
        rd: Register::new(value, 16, 0b1111),
        rs: Register::new(value, 8, 0b1111),
        rm: Register::new(value, 0, 0b1111),
    }
}

#[cfg(test)]
mod tests {


    use super::{
        ArmOperand,
        BitState,
        Register,
        get_operand,
    };

    #[test]
    fn test_get_operand() {
        let value = 0b0000_0000_0001_1111_0000_1111_1001_1111;

        assert_eq!(
            ArmOperand::MUL {
                s: BitState::SET,
                rd: Register::from(0b1111),
                rs: Register::from(0b1111),
                rm: Register::from(0b1111),
            },
            get_operand(value)
        );
    }

    #[test]
    #[should_panic]
    fn test_get_operand_sbz() {
        let value = 0b0000_0000_0001_1111_1111_1111_1001_1111;
        get_operand(value);
    }
}
