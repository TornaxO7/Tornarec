use crate::cpus::general::instruction::decode::DecodeData;

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoftwareInterrupt {
    immediate: u8,
}

impl<'a> From<DecodeData<'a>> for SoftwareInterrupt {
    fn from(data: DecodeData<'a>) -> Self {
        let immediate = u8::try_from(data.instruction.val & 0b1111_1111).unwrap();
        Self { immediate }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DecodeData,
        SoftwareInterrupt,
    };

    use crate::{
        NintendoDS,
        cpus::general::Instruction,
    };

    #[test]
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b1101_1111_1100_1000);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = SoftwareInterrupt::from(data);

        let expected_value = SoftwareInterrupt {
            immediate: 0b1100_1000,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
