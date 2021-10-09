use crate::cpus::general::instruction::Instruction;

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingRegister {
    opcode: u8,
    rm_rs: u8,
    rd_rn: u8,
}

impl From<&Instruction> for DataProcessingRegister {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 6) & 0b1111).unwrap();
        let rm_rs = u8::try_from((instruction_val >> 3) & 0b111).unwrap();
        let rd_rn = u8::try_from(instruction_val & 0b111).unwrap();
        Self {opcode, rm_rs, rd_rn}
    }
}

#[cfg(test)]
mod tests {
    use super::{DataProcessingRegister, Instruction};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b010000_1111_101_010);
        let value = DataProcessingRegister::from(&instruction);

        let expected_value = DataProcessingRegister {
            opcode: 0b1111,
            rm_rs: 0b101,
            rd_rn: 0b010,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
