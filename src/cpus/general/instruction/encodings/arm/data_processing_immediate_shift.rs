use crate::cpus::general::{
    bit_state::BitState,
    instruction::{
        decode::DecodeData,
        encodings::encoding_fields::DataProcessingInstruction,
    },
    register::NormalizedRegister,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingImmediateShift {
    pub opcode: DataProcessingInstruction,
    pub s_flag: BitState,
    pub rn: NormalizedRegister,
    pub rd: NormalizedRegister,
    pub shift_imm: u8,
    pub shift: u8,
    pub rm: NormalizedRegister,
}

impl<'a> From<DecodeData<'a>> for DataProcessingImmediateShift {
    fn from(decode_data: DecodeData<'a>) -> Self {
        let instruction_val = decode_data.instruction.get_value_as_u32();

        let opcode = DataProcessingInstruction::from((instruction_val >> 21) & 0b1111);
        let s_flag = BitState::from(instruction_val >> 20);
        let rn = NormalizedRegister::from((instruction_val >> 16) & 0b1111);
        let rd = NormalizedRegister::from((instruction_val >> 12) & 0b1111);
        let shift_amount = u8::try_from((instruction_val >> 7) & 0b1_1111).unwrap();
        let shift = u8::try_from((instruction_val >> 5) & 0b11).unwrap();
        let rm = NormalizedRegister::from(instruction_val & 0b1111);

        Self {
            opcode,
            s_flag,
            rn,
            rd,
            shift_imm: shift_amount,
            shift,
            rm,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        DataProcessingImmediateShift,
        DataProcessingInstruction,
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
        let instruction = Instruction::from(0b0000_000_1010_1_1010_0101_11100_10_0_1001);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = DataProcessingImmediateShift::from(data);

        let expected_value = DataProcessingImmediateShift {
            opcode: DataProcessingInstruction::CMP,
            s_flag: BitState::Set,
            rn: NormalizedRegister::from(RegisterName::R10),
            rd: NormalizedRegister::from(RegisterName::R5),
            shift_imm: 0b11100,
            shift: 0b10,
            rm: NormalizedRegister::from(RegisterName::R9),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
