use crate::{
    cpus::general::instruction::arm::types::{
        sbz,
        Register,
    },
    ram::Word,
};

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
    sbz(value, 8, 0b1111);

    ArmOperand::Saturating {
        rn: Register::new(value, 16, 0b1111),
        rd: Register::new(value, 12, 0b1111),
        rm: Register::new(value, 0, 0b1111),
    }
}

#[cfg(test)]
mod tests {

    use super::get_operand;

    #[test]
    #[should_panic]
    fn test_get_operand_sbz() {
        let value = 0b0000_0001_0010_1111_1111_1111_0101_1111;
        get_operand(value);
    }
}
