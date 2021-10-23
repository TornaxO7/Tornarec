use crate::cpus::general::{
    bit_state::BitState,
    instruction::{
        decode::DecodeData,
        encodings::encoding_fields::DataProcessingInstruction,
    },
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingRegisterShift {
    opcode: DataProcessingInstruction,
    s_flag: BitState,
    rn: u8,
    rd: u8,
    rs: u8,
    shift: u8,
    rm: u8,
}

impl<'a> From<DecodeData<'a>> for DataProcessingRegisterShift {
    fn from(data: DecodeData<'a>) -> Self {
        let opcode = DataProcessingInstruction::from((data.instruction.val >> 21) & 0b1111);
        let s_flag = BitState::from(data.instruction.val >> 20);
        let rn = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let rs = u8::try_from((data.instruction.val >> 8) & 0b1111).unwrap();
        let shift = u8::try_from((data.instruction.val >> 5) & 0b11).unwrap();
        let rm = u8::try_from(data.instruction.val & 0b1111).unwrap();

        Self {
            opcode,
            s_flag,
            rn,
            rd,
            rs,
            shift,
            rm,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DataProcessingInstruction,
        DataProcessingRegisterShift,
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
    fn test_from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b0000_000_1111_1_1010_0101_0110_0_11_1_1001);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = DataProcessingRegisterShift::from(data);

        let expected_value = DataProcessingRegisterShift {
            opcode: DataProcessingInstruction::MVN,
            s_flag: BitState::Set,
            rn: NormalizedRegister::from(RegisterName::R10),
            rd: NormalizedRegister::from(RegisterName::R5),
            rs: NormalizedRegister::from(RegisterName::R6),
            shift: 0b11,
            rm: NormalizedRegister::from(RegisterName::R9),
        };

        assert_eq!(value, expected_value);
    }
}
