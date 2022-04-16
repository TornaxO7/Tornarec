use crate::{
    cpus::general::instruction::arm::Register,
    ram::Word,
};

use super::ArmOperand;

use std::convert::TryFrom;

pub fn get_operand(value: Word) -> ArmOperand {
    let sbo1 = (value >> 16) & 0b1111;
    let rd = Register::try_from((value >> 12) & 0b1111).unwrap();
    let sbo2 = (value >> 8) & 0b1111;
    let rm = Register::try_from(value & 0b1111).unwrap();

    if sbo1 != 0b1111 || sbo2 != 0b1111 {
        unreachable!();
    }

    ArmOperand::CLZ { rd, rm }
}

#[cfg(test)]
mod tests {
    use crate::cpus::general::instruction::arm::{
        operand::{
            breakpoint::get_operand,
            ArmOperand,
        },
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
