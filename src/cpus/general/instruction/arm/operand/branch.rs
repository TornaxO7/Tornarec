use crate::{
    cpus::general::instruction::arm::{
        types::{
            sbo,
            Register,
        },
        BitState,
    },
    ram::Word,
};

use super::ArmOperand;

pub fn normal(value: Word) -> ArmOperand {
    let immed24 = value & 0b1111_1111_1111_1111_1111_1111;
    ArmOperand::Branch(immed24)
}

pub fn register(value: Word) -> ArmOperand {
    // example 170
    // Affected: BX, BLX(2)
    sbo(value, 8, 0b1111_1111_1111);

    let rm = Register::new(value, 0, 0b1111);
    ArmOperand::BRegister(rm)
}

pub fn blx1(value: Word) -> ArmOperand {
    let h = BitState::new(value, 24);
    let immed24 = value & 0b1111_1111_1111_1111_1111_1111;

    ArmOperand::BLX1 { h, immed24 }
}

#[cfg(test)]
mod tests {
    use crate::cpus::general::instruction::arm::{
        operand::{
            branch::{
                blx1,
                normal,
                register,
            },
            ArmOperand,
        },
        types::Register,
        BitState,
    };

    #[test]
    fn test_normal() {
        let value = 0b0000_1011_1111_1111_1111_1111_1111_1111;

        assert_eq!(ArmOperand::Branch((1 << 24) - 1), normal(value));
    }

    #[test]
    fn test_register() {
        let value = 0b0000_0001_0010_1111_1111_1111_0011_1111;

        assert_eq!(
            ArmOperand::BRegister(Register::from(0b1111)),
            register(value)
        );
    }

    #[test]
    #[should_panic]
    fn test_register_sbo() {
        let value = 0b0000_0001_0010_0000_0000_0000_0011_1111;
        register(value);
    }

    #[test]
    fn teste_blx1() {
        let value = 0b1111_1011_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmOperand::BLX1 {
                h: BitState::SET,
                immed24: 0b1111_1111_1111_1111_1111_1111
            },
            blx1(value)
        );
    }
}
