use crate::cpus::general::{
    instruction::Instruction,
    BitState,
    register::NormalizedRegister,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreToFromStack {
    l_flag: BitState,
    rd: NormalizedRegister,
    sp_relative_offset: u8,
}

impl From<&Instruction> for LoadStoreToFromStack {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let l_flag = BitState::from(instruction_val >> 11);
        let rd = NormalizedRegister::from((instruction_val >> 8) & 0b111);
        let sp_relative_offset = u8::try_from(instruction_val & 0b1111_1111).unwrap();
        Self {l_flag, rd, sp_relative_offset}
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadStoreToFromStack, Instruction, BitState, NormalizedRegister};

    use crate::cpus::general::register::RegisterName;

    #[test]
    fn from() {
        let instruction = Instruction::from(0b1001_1_101_1100_0011);
        let value = LoadStoreToFromStack::from(&instruction);

        let expected_value = LoadStoreToFromStack {
            l_flag: BitState::Set,
            rd: NormalizedRegister::from(RegisterName::R5),
            sp_relative_offset: 0b1100_0011,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
