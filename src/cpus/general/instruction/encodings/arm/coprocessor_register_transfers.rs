use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
    register::NormalizedRegister,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoprocessorRegisterTransfers {
    opcode1: u8,
    l_flag: BitState,
    crn: u8,
    rd: NormalizedRegister,
    cp_num: u8,
    opcode2: u8,
    crm: u8,
}

impl From<DecodeData> for CoprocessorRegisterTransfers {
    fn from(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();

        let opcode1 = u8::try_from((instruction_val >> 21) & 0b111).unwrap();
        let l_flag = BitState::from(instruction_val >> 20);
        let crn = u8::try_from((instruction_val >> 16) & 0b1111).unwrap();
        let rd = NormalizedRegister::from((instruction_val >> 12) & 0b1111);
        let cp_num = u8::try_from((instruction_val >> 8) & 0b1111).unwrap();
        let opcode2 = u8::try_from((instruction_val >> 5) & 0b111).unwrap();
        let crm = u8::try_from(instruction_val & 0b1111).unwrap();
        Self {
            opcode1,
            l_flag,
            crn,
            rd,
            cp_num,
            opcode2,
            crm,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        CoprocessorRegisterTransfers,
        DecodeData,
    };

    use crate::{
        NintendoDS,
        cpus::general::instruction::Instruction,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b0000_1110_111_1_1110_1100_1000_101_1_0101);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = CoprocessorRegisterTransfers::from(data);

        let expected_value = CoprocessorRegisterTransfers {
            opcode1: 0b111,
            l_flag: BitState::Set,
            crn: 0b1110,
            rd: 0b1100,
            cp_num: 0b1000,
            opcode2: 0b101,
            crm: 0b0101,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
