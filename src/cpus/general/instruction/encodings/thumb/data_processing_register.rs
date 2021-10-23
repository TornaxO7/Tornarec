use crate::cpus::general::{
    instruction::decode::DecodeData,
    register::NormalizedRegister,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingRegister {
    opcode: u8,
    rm_rs: NormalizedRegister,
    rd_rn: NormalizedRegister,
}

impl From<DecodeData> for DataProcessingRegister {
    fn from(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();

        let opcode = u8::try_from((instruction_val >> 6) & 0b1111).unwrap();
        let rm_rs = NormalizedRegister::from((instruction_val >> 3) & 0b111);
        let rd_rn = NormalizedRegister::from(instruction_val & 0b111);
        Self {
            opcode,
            rm_rs,
            rd_rn,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DataProcessingRegister,
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
        let instruction = Instruction::from(0b010000_1111_101_010);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = DataProcessingRegister::from(data);

        let expected_value = DataProcessingRegister {
            opcode: 0b1111,
             rm_rs: NormalizedRegister::from(RegisterName::R5),
             rd_rn: NormalizedRegister::from(RegisterName::R2),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
