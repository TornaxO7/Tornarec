use crate::cpus::general::instruction::Instruction;

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadStoreRegisterOffset {
    opcode: u8,
    rm: u8,
    rn: u8,
    rd: u8,
}

impl From<&Instruction> for LoadStoreRegisterOffset {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 9) & 0b111).unwrap();
        let rm = u8::try_from((instruction_val >> 6) & 0b111).unwrap();
        let rn = u8::try_from((instruction_val >> 3) & 0b111).unwrap();
        let rd = u8::try_from(instruction_val & 0b111).unwrap();
        Self {opcode, rm, rn, rd}
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadStoreRegisterOffset, Instruction};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b0101_111_110_100_101);
        let value = LoadStoreRegisterOffset::from(&instruction);

        let expected_value = LoadStoreRegisterOffset {
            opcode: 0b111,
            rm: 0b110,
            rn: 0b100,
            rd: 0b101,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
