use crate::cpus::general::{
    instruction::decode::DecodeData,
    register::NormalizedRegister,
};

use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSubtractCompareMoveImmediate {
    opcode: u8,
    rd_rn: NormalizedRegister,
    immediate: u8,
}

impl From<DecodeData> for AddSubtractCompareMoveImmediate {
    fn from(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 11) & 0b11).unwrap();
        let rd_rn = NormalizedRegister::from((instruction_val >> 8) & 0b111);
        let immediate = u8::try_from(instruction_val & 0b1111_1111).unwrap();
        Self {opcode, rd_rn, immediate}
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AddSubtractCompareMoveImmediate,
        DecodeData,
        NormalizedRegister
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
        let instruction = Instruction::from(0b001_11_110_1010_0101);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = AddSubtractCompareMoveImmediate::from(data);

        let expected_value = AddSubtractCompareMoveImmediate {
            opcode: 0b11,
            rd_rn: NormalizedRegister::from(RegisterName::R6),
            immediate: 0b1010_0101,
        };

        assert_eq!(value, expected_value, "{:#?}, {:#?}", &value, &expected_value);
    }
}
