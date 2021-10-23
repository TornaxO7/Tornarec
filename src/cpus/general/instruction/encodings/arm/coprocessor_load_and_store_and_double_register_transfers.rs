use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoprocessorLoadAndStoreAndDoubleRegisterTransfers {
    p_flag: BitState,
    u_flag: BitState,
    n_flag: BitState,
    w_flag: BitState,
    l_flag: BitState,
    rn: u8,
    crd: u8,
    cp_num: u8,
    offset: u8,
}

impl<'a> From<DecodeData<'a>> for CoprocessorLoadAndStoreAndDoubleRegisterTransfers {
    fn from(data: DecodeData<'a>) -> Self {
        let p_flag = BitState::from(data.instruction.val >> 24);
        let u_flag = BitState::from(data.instruction.val >> 23);
        let n_flag = BitState::from(data.instruction.val >> 22);
        let w_flag = BitState::from(data.instruction.val >> 21);
        let l_flag = BitState::from(data.instruction.val >> 20);
        let rn = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let crd = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let cp_num = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let offset = u8::try_from(data.instruction.val & 0b1111_1111).unwrap();

        Self {
            p_flag,
            u_flag,
            n_flag,
            w_flag,
            l_flag,
            rn,
            crd,
            cp_num,
            offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        CoprocessorLoadAndStoreAndDoubleRegisterTransfers,
        DecodeData,
    };

    use crate::{
        cpus::general::Instruction,
        ram::Address,
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            address: Address::default(),
            val: 0b0000_110_1_0_1_0_1_1111_1110_1100_1010_1010,
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = CoprocessorLoadAndStoreAndDoubleRegisterTransfers::from(data);

        let expected_value = CoprocessorLoadAndStoreAndDoubleRegisterTransfers {
            p_flag: BitState::Set,
            u_flag: BitState::Unset,
            n_flag: BitState::Set,
            w_flag: BitState::Unset,
            l_flag: BitState::Set,
            rn: 0b1111,
            crd: 0b1110,
            cp_num: 0b1100,
            offset: 0b1010_1010,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
