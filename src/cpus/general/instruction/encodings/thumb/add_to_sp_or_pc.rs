use crate::cpus::general::{
    instruction::decode::DecodeData,
    register::NormalizedRegister,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddToSpOrPc {
    sp: BitState,
    rd: NormalizedRegister,
    immediate: u8,
}

impl From<DecodeData> for AddToSpOrPc {
    fn from(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();

        let sp = BitState::from(instruction_val >> 11);
        let rd = NormalizedRegister::from((instruction_val >> 8) & 0b111);
        let immediate = u8::try_from(instruction_val & 0b1111_1111).unwrap();
        Self { sp, rd, immediate }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AddToSpOrPc,
        BitState,
        DecodeData,
        NormalizedRegister,
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
        let instruction = Instruction::from(0b1011_1_101_1100_1000);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = AddToSpOrPc::from(data);

        let expected_value = AddToSpOrPc {
            sp: BitState::Set,
            rd: NormalizedRegister::from(RegisterName::R5),
            immediate: 0b1100_1000,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
