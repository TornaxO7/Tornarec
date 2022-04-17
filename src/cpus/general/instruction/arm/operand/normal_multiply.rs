use crate::{
    cpus::general::instruction::arm::{types::{Register, sbz}, BitState},
    ram::Word,
};

use super::ArmOperand;

pub fn get_mla_operand(value: Word) -> ArmOperand {
    let rn = Register::new(value, 12, 0b1111);
    ArmOperand::NormalMultiply {
        s: BitState::new(value, 20),
        rd: Register::new(value, 16, 0b1111),
        rs: Register::new(value, 8, 0b1111),
        rm: Register::new(value, 0, 0b1111),
        mul_type: NormalMultiplyType::MLA(rn),
    }
}

pub fn get_mul_operand(value: Word) -> ArmOperand {
    sbz(value, 12, 0b1111);
    ArmOperand::NormalMultiply {
        s: BitState::new(value, 20),
        rd: Register::new(value, 16, 0b1111),
        rs: Register::new(value, 8, 0b1111),
        rm: Register::new(value, 0, 0b1111),
        mul_type: NormalMultiplyType::MUL,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NormalMultiplyType {
    MLA(Register),
    MUL,
}

#[cfg(test)]
mod tests {

    use super::{
        get_mla_operand,
        get_mul_operand,
        ArmOperand,
        Register,
        BitState,
        NormalMultiplyType,
    };

    #[test]
    fn test_get_operand() {
        let value = 0b0000_0000_0011_1111_1111_1111_1001_1111;

        assert_eq!(
            ArmOperand::NormalMultiply {
                s: BitState::SET,
                rd: Register::from(0b1111),
                rs: Register::from(0b1111),
                rm: Register::from(0b1111),
                mul_type: NormalMultiplyType::MLA(Register::from(0b1111)),
            },
            get_mla_operand(value)
        );
    }

    #[test]
    fn test_get_mul_operand() {
        let value = 0b0000_0000_0001_1111_0000_1111_1001_1111;

        assert_eq!(
            ArmOperand::NormalMultiply {
                s: BitState::SET,
                rd: Register::from(0b1111),
                rs: Register::from(0b1111),
                rm: Register::from(0b1111),
                mul_type: NormalMultiplyType::MUL,
            },
            get_mul_operand(value)
        );
    }

    #[test]
    #[should_panic]
    fn test_get_mul_operand_sbz() {
        let value = 0b0000_0000_0001_1111_1111_1111_1001_1111;
        get_mul_operand(value);
    }
}
