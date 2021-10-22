use crate::cpus::general::{instruction::Instruction, register::NormalizedRegister, BitState};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSubtractImmediate {
    opc: BitState,
    immediate: u8,
    rn: NormalizedRegister,
    rd: NormalizedRegister,
}

impl From<&Instruction> for AddSubtractImmediate {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opc = BitState::from(instruction_val >> 9);
        let immediate = u8::try_from((instruction_val >> 6) & 0b111).unwrap();
        let rn = NormalizedRegister::from((instruction_val >> 3) & 0b111);
        let rd = NormalizedRegister::from(instruction_val & 0b111);
        Self {
            opc,
            immediate,
            rn,
            rd,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AddSubtractImmediate, BitState, Instruction, NormalizedRegister};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b000_111_1_111_110_100);
        let value = AddSubtractImmediate::from(&instruction);

        let expected_value = AddSubtractImmediate {
            opc: BitState::Set,
            immediate: 0b111,
            rn: NormalizedRegister::from(RegisterName::R6),
            rd: NormalizedRegister::from(RegisterName::R4),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
