use crate::cpus::general::{
    instruction::Instruction,
    register::NormalizedRegister,
    BitState,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSubtractRegister {
    opc: BitState,
    rm:  NormalizedRegister,
    rn:  NormalizedRegister,
    rd:  NormalizedRegister,
}

impl From<&Instruction> for AddSubtractRegister {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opc = BitState::from(instruction_val >> 9);
        let rm = NormalizedRegister::from((instruction_val >> 6) & 0b111);
        let rn = NormalizedRegister::from((instruction_val >> 3) & 0b111);
        let rd = NormalizedRegister::from(instruction_val & 0b111);
        Self { opc, rm, rn, rd }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AddSubtractRegister,
        BitState,
        Instruction,
        NormalizedRegister,
    };

    use crate::cpus::general::register::RegisterName;

    #[test]
    fn from() {
        let instruction = Instruction::from(0b000_11_0_1_111_110_100);
        let value = AddSubtractRegister::from(&instruction);

        let expected_value = AddSubtractRegister {
            opc: BitState::Set,
            rm:  NormalizedRegister::from(RegisterName::R7),
            rn:  NormalizedRegister::from(RegisterName::R6),
            rd:  NormalizedRegister::from(RegisterName::R4),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
