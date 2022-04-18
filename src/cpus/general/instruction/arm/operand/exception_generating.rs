use crate::ram::Word;

use super::ArmOperand;

use std::convert::TryFrom;

pub fn get_swi(value: Word) -> ArmOperand {
    let immed24 = value & 0b1111_1111_1111_1111_1111_1111;
    ArmOperand::SWI(immed24)
}

pub fn get_bkpt(value: Word) -> ArmOperand {
    let immed1 = u16::try_from((value >> 8) & 0b1111_1111_1111).unwrap();
    let immed2 = u8::try_from(value & 0b1111).unwrap();

    ArmOperand::BKPT { immed1, immed2 }
}

#[cfg(test)]
mod tests {

    use super::{
        ArmOperand,
        get_swi,
        get_bkpt,
    };

    #[test]
    fn test_get_swi() {
        let value = 0b0000_1111_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmOperand::SWI(0b1111_1111_1111_1111_1111_1111),
            get_swi(value)
        );
    }

    #[test]
    fn test_bkpt() {
        let value = 0b1110_0001_0010_1111_1111_1111_0111_1111;

        let operand = get_bkpt(value);
        let expected = ArmOperand::BKPT {
                immed1: 0b1111_1111_1111,
                immed2: 0b1111
        };
        assert_eq!(expected, operand, "{:#?}, {:#?}", expected, operand);
    }
}
