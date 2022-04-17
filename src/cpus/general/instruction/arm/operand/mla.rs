use crate::{
    cpus::general::instruction::arm::{types::Register, BitState},
    ram::Word,
};

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
    ArmOperand::MLA {
        s: BitState::new(value, 20),
        rd: Register::new(value, 16, 0b1111),
        rn: Register::new(value, 12, 0b1111),
        rs: Register::new(value, 8, 0b1111),
        rm: Register::new(value, 0, 0b1111),
    }
}

#[cfg(test)]
mod tests {

    use super::{
        get_operand,
        ArmOperand,
        Register,
        BitState,
    };

    #[test]
    fn test_get_operand() {
        let value = 0b0000_0000_0011_1111_1111_1111_1001_1111;

        assert_eq!(
            ArmOperand::MLA {
                s: BitState::SET,
                rd: Register::from(0b1111),
                rn: Register::from(0b1111),
                rs: Register::from(0b1111),
                rm: Register::from(0b1111),
            },
            get_operand(value)
        );
    }
}
