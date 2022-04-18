use crate::{ram::Word, cpus::general::instruction::arm::types::{sbo, Register}};

use super::ArmOperand;

pub fn get_clz(value: Word) -> ArmOperand {
    sbo(value, 16, 0b1111);
    sbo(value, 8, 0b1111);

    ArmOperand::CLZ {
        rd: Register::new(value, 12, 0b1111),
        rm: Register::new(value, 0, 0b1111),
    }
}

#[cfg(test)]
mod tests {

    use super::{
        get_clz,
        ArmOperand,
        Register,
    };

    #[test]
    fn test_get_clz() {
        let value = 0b0000_0001_0110_1111_1111_1111_0001_1111;

        assert_eq!(
            ArmOperand::CLZ {
                rd: Register::from(0b1111),
                rm: Register::from(0b1111),
            },
            get_clz(value)
        );
    }

    #[test]
    #[should_panic]
    fn test_get_clz_sbo1() {
        let value = 0b0000_0001_0110_0000_1111_1111_0001_1111;
        get_clz(value);
    }

    #[test]
    #[should_panic]
    fn test_get_clz_sbo2() {
        let value = 0b0000_0001_0110_1111_1111_0000_0001_1111;
        get_clz(value);
    }
}
