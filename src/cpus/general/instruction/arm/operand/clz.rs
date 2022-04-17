use crate::{
    cpus::general::instruction::arm::types::{
        sbo,
        Register,
    },
    ram::Word,
};

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
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
        get_operand,
        ArmOperand,
        Register,
    };

    #[test]
    fn test_get_operand() {
        let value = 0b0000_0001_0110_1111_1111_1111_0001_1111;

        assert_eq!(
            ArmOperand::CLZ {
                rd: Register::from(0b1111),
                rm: Register::from(0b1111),
            },
            get_operand(value)
        );
    }
}
