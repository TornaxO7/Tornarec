use crate::cpus::general::{
    instruction::decode::DecodeData,
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdjustStackPointer {
    opc: BitState,
    immediate: u8,
}

impl From<DecodeData> for AdjustStackPointer {
    fn from(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();

        let opc = BitState::from(instruction_val >> 7);
        let immediate = u8::try_from(instruction_val & 0b111_1111).unwrap();
        Self { opc, immediate }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AdjustStackPointer,
        BitState,
        DecodeData,
    };
    
    use crate::{
        NintendoDS,
        cpus::general::Instruction,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b1011_0000_1_100_1000);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = AdjustStackPointer::from(data);

        let expected_value = AdjustStackPointer {
            opc: BitState::Set,
            immediate: 0b100_1000,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
