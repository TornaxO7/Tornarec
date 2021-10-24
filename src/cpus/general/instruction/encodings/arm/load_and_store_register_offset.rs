use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadAndStoreRegisterOffset {
    p_flag: BitState,
    u_flag: BitState,
    b_flag: BitState,
    w_flag: BitState,
    l_flag: BitState,
    rn: u8,
    rd: u8,
    shift_amount: u8,
    shift: u8,
    rm: u8,
}

impl<'a> From<DecodeData<'a>> for LoadAndStoreRegisterOffset {
    fn from(data: DecodeData<'a>) -> Self {
        let p_flag = BitState::from(data.instruction.val >> 24);
        let u_flag = BitState::from(data.instruction.val >> 23);
        let b_flag = BitState::from(data.instruction.val >> 22);
        let w_flag = BitState::from(data.instruction.val >> 21);
        let l_flag = BitState::from(data.instruction.val >> 20);
        let rn = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let shift_amount = u8::try_from((data.instruction.val >> 7) & 0b1_1111).unwrap();
        let shift = u8::try_from((data.instruction.val >> 5) & 0b11).unwrap();
        let rm = u8::try_from(data.instruction.val & 0b1111).unwrap();
        Self {
            p_flag,
            u_flag,
            b_flag,
            w_flag,
            l_flag,
            rn,
            rd,
            shift_amount,
            shift,
            rm,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DecodeData,
        LoadAndStoreRegisterOffset,
    };

    use crate::{
        cpus::general::Instruction,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b0000_011_1_0_1_0_1_1100_0011_11100_01_0_1010,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = LoadAndStoreRegisterOffset::from(data);

        let expected_value = LoadAndStoreRegisterOffset {
            p_flag: BitState::Set,
            u_flag: BitState::Unset,
            b_flag: BitState::Set,
            w_flag: BitState::Unset,
            l_flag: BitState::Set,
            rn: 0b1100,
            rd: 0b0011,
            shift_amount: 0b11100,
            shift: 0b01,
            rm: 0b1010,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
