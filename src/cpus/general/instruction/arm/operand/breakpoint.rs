use crate::ram::Word;

use super::ArmOperand;

use std::convert::TryFrom;

pub fn get_operand(value: Word) -> ArmOperand {
    let immed1 = u16::try_from((value >> 8) & 0b1111_1111_1111).unwrap();
    let immed2 = u8::try_from(value & 0b1111).unwrap();

    ArmOperand::BKPT { immed1, immed2 }
}

#[cfg(test)]
mod tests {

    use super::{
        get_operand,
        ArmOperand,
    };

    #[test]
    fn test_bkpt() {
        let value = 0b1110_0001_0010_1111_1111_1111_0111_1111;

        let operand = get_operand(value);
        let expected = ArmOperand::BKPT {
                immed1: 0b1111_1111_1111,
                immed2: 0b1111
        };
        assert_eq!(expected, operand, "{:#?}, {:#?}", expected, operand);
    }
}
