use crate::cpus::general::{
    instruction::Instruction,
    register::RegisterName,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreRegisterOffset {
    opcode: u8,
    rm: RegisterName,
    rn: RegisterName,
    rd: RegisterName,
}

impl From<&Instruction> for LoadStoreRegisterOffset {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 9) & 0b111).unwrap();
        let rm = RegisterName::from((instruction_val >> 6) & 0b111);
        let rn = RegisterName::from((instruction_val >> 3) & 0b111);
        let rd = RegisterName::from(instruction_val & 0b111);
        Self {opcode, rm, rn, rd}
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadStoreRegisterOffset, Instruction, RegisterName};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b0101_111_110_100_101);
        let value = LoadStoreRegisterOffset::from(&instruction);

        let expected_value = LoadStoreRegisterOffset {
            opcode: 0b111,
            rm: RegisterName::R6,
            rn: RegisterName::R4,
            rd: RegisterName::R5,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
