use crate::cpus::general::{
    instruction::Instruction,
    BitState,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSubtractRegister {
    opc: BitState,
    rm: u8,
    rn: u8,
    rd: u8,
}

impl From<&Instruction> for AddSubtractRegister {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opc = BitState::from(instruction_val >> 9);
        let rm = u8::try_from((instruction_val >> 6) & 0b111).unwrap();
        let rn = u8::try_from((instruction_val >> 3) & 0b111).unwrap();
        let rd = u8::try_from(instruction_val & 0b111).unwrap();
        Self {opc, rm, rn, rd}
    }
}

#[cfg(test)]
mod tests {
    use super::{AddSubtractRegister, BitState, Instruction};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b000_11_0_1_111_110_100);
        let value = AddSubtractRegister::from(&instruction);

        let expected_value = AddSubtractRegister {
            opc: BitState::Set,
            rm: 0b111,
            rn: 0b110,
            rd: 0b100,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
