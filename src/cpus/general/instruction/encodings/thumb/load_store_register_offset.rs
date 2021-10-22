use crate::cpus::general::{
    instruction::decode::DecodeData,
    register::NormalizedRegister,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreRegisterOffset {
    opcode: u8,
    rm: NormalizedRegister,
    rn: NormalizedRegister,
    rd: NormalizedRegister,
}

impl<'a> From<DecodeData<'a>> for LoadStoreRegisterOffset {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 9) & 0b111).unwrap();
        let rm = NormalizedRegister::from((instruction_val >> 6) & 0b111);
        let rn = NormalizedRegister::from((instruction_val >> 3) & 0b111);
        let rd = NormalizedRegister::from(instruction_val & 0b111);
        Self {opcode, rm, rn, rd}
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadStoreRegisterOffset, Instruction, NormalizedRegister};

    use crate::cpus::general::register::RegisterName;

    #[test]
    fn from() {
        let instruction = Instruction::from(0b0101_111_110_100_101);
        let value = LoadStoreRegisterOffset::from(&instruction);

        let expected_value = LoadStoreRegisterOffset {
            opcode: 0b111,
            rm: NormalizedRegister::from(RegisterName::R6),
            rn: NormalizedRegister::from(RegisterName::R4),
            rd: NormalizedRegister::from(RegisterName::R5),
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
