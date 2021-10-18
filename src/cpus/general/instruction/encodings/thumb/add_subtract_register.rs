use crate::cpus::general::{
    instruction::Instruction,
    BitState,
    register::RegisterName,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSubtractRegister {
    opc: BitState,
    rm: RegisterName,
    rn: RegisterName,
    rd: RegisterName,
}

impl From<&Instruction> for AddSubtractRegister {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let opc = BitState::from(instruction_val >> 9);
        let rm = RegisterName::from((instruction_val >> 6) & 0b111);
        let rn = RegisterName::from((instruction_val >> 3) & 0b111);
        let rd = RegisterName::from(instruction_val & 0b111);
        Self {opc, rm, rn, rd}
    }
}

#[cfg(test)]
mod tests {
    use super::{AddSubtractRegister, BitState, Instruction, RegisterName};

    #[test]
    fn from() {
        let instruction = Instruction::from(0b000_11_0_1_111_110_100);
        let value = AddSubtractRegister::from(&instruction);

        let expected_value = AddSubtractRegister {
            opc: BitState::Set,
            rm: RegisterName::R7,
            rn: RegisterName::R6,
            rd: RegisterName::R4,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
