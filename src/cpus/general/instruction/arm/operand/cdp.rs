use crate::{
    cpus::general::instruction::arm::types::Register,
    ram::Word,
};

use super::ArmOperand;

use std::convert::TryFrom;

pub fn get_operand(value: Word) -> ArmOperand {
    let opcode1 = u8::try_from((value >> 20) & 0b1111).unwrap();
    let num = u8::try_from((value >> 8) & 0b1111).unwrap();
    let opcode2 = u8::try_from((value >> 5) & 0b111).unwrap();

    ArmOperand::CDP {
        opcode1,
        crn: Register::new(value, 16, 0b1111),
        crd: Register::new(value, 12, 0b1111),
        num,
        opcode2,
        crm: Register::new(value, 0, 0b1111),
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
        let value = 0b0000_1110_1111_1111_1111_1111_1110_1111;

        let operand = get_operand(value);
        let expected = ArmOperand::CDP {
            opcode1: u8::from(0b1111),
            crn: Register::from(0b1111),
            crd: Register::from(0b1111),
            num: 0b1111,
            opcode2: 0b111,
            crm: Register::from(0b1111),
        };
        assert_eq!(expected, operand, "{:#?}, {:#?}", expected, operand);
    }
}
