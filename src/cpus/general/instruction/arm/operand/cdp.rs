use crate::{
    cpus::general::instruction::arm::{
        CPNum,
        CPRegister,
    },
    ram::Word,
};

use super::ArmOperand;

use std::convert::TryFrom;

pub fn get_operand(value: Word) -> ArmOperand {
    let opcode1 = u8::try_from((value >> 20) & 0b1111).unwrap();
    let crn = CPRegister::try_from((value >> 16) & 0b1111).unwrap();
    let crd = CPRegister::try_from((value >> 12) & 0b1111).unwrap();
    let num = CPNum::try_from((value >> 8) & 0b1111).unwrap();
    let opcode2 = u8::try_from((value >> 5) & 0b111).unwrap();
    let crm = CPRegister::try_from(value & 0b1111).unwrap();

    ArmOperand::CDP {
        opcode1,
        crn,
        crd,
        num,
        opcode2,
        crm,
    }
}

#[cfg(test)]
mod tests {
    use crate::cpus::general::instruction::arm::{
        operand::{
            breakpoint::get_operand,
            ArmOperand,
        },
        CPNum,
        CPRegister,
    };

    #[test]
    fn test_get_operand() {
        let value = 0b0000_1110_1111_1111_1111_1111_1110_1111;

        assert_eq!(
            ArmOperand::CDP {
                opcode1: u8::from(0b1111),
                crn: CPRegister::from(0b1111),
                crd: CPRegister::from(0b1111),
                num: CPNum::from(0b1111),
                opcode2: u8::from(0b111),
                crm: CPRegister::from(01111),
            },
            get_operand(value)
        );
    }
}
