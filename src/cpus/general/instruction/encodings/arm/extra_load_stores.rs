use crate::cpus::general::{
    bit_state::BitState,
    instruction::decode::DecodeData,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtraLoadAndStores {
    p_flag: BitState,
    u_flag: BitState,
    b_flag: BitState,
    w_flag: BitState,
    l_flag: BitState,
    rn: u8,
    rd: u8,
    rs: u8,
    op1: u8,
    rm: u8,
}

impl<'a> From<DecodeData<'a>> for ExtraLoadAndStores {
    fn from(data: DecodeData<'a>) -> Self {
        let p_flag = BitState::from(data.instruction.val >> 24);
        let u_flag = BitState::from(data.instruction.val >> 23);
        let b_flag = BitState::from(data.instruction.val >> 22);
        let w_flag = BitState::from(data.instruction.val >> 21);
        let l_flag = BitState::from(data.instruction.val >> 20);
        let rn = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let rs = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let op1 = u8::try_from((data.instruction.val >> 5) & 0b11).unwrap();
        let rm = u8::try_from(data.instruction.val & 0b1111).unwrap();
        Self {
            p_flag,
            u_flag,
            b_flag,
            w_flag,
            l_flag,
            rn,
            rd,
            rs,
            op1,
            rm,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DecodeData,
        ExtraLoadAndStores,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b0000_000_1_0_1_0_1_1100_0011_1010_1_11_1_0101,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = ExtraLoadAndStores::from(data);

        let expected_value = ExtraLoadAndStores {
            p_flag: BitState::Set,
            u_flag: BitState::Unset,
            b_flag: BitState::Set,
            w_flag: BitState::Unset,
            l_flag: BitState::Set,
            rn: 0b1100,
            rd: 0b0011,
            rs: 0b1010,
            op1: 0b11,
            rm: 0b0101,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
