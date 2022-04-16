use crate::ram::Word;

use super::ArmOperand;

pub fn get_operand(value: Word) -> ArmOperand {
    let immed24 = value >> 0b1111_1111_1111_1111_1111_1111;
    ArmOperand::SWI(immed24)
}

#[cfg(test)]
mod tests {
    use crate::cpus::general::instruction::arm::operand::{
        breakpoint::get_operand,
        ArmOperand,
    };

    #[test]
    fn test_swi_operand() {
        let value = 0b0000_1111_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmOperand::SWI(0b1111_1111_1111_1111_1111_1111),
            get_operand(value)
        );
    }
}
