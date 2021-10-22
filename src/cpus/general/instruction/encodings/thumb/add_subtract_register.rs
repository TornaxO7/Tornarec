use crate::cpus::general::{
    instruction::decode::DecodeData,
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

impl<'a> From<DecodeData<'a>> for AddSubtractRegister {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

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
        NormalizedRegister,
        DecodeData,
    };

    use crate::{
        cpus::general::{
            register::RegisterName,
            Instruction,
        },
        NintendoDS,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b000_11_0_1_111_110_100);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = AddSubtractRegister::from(data);

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
