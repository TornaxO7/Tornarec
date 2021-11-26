use crate::cpus::general::{
    bit_state::BitState,
    instruction::{
        decode::DecodeData,
        encodings::encoding_fields::{
            DataProcessingInstruction,
            ShifterOperand,
        },
    },
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingRegisterShift {
    pub opcode: DataProcessingInstruction,
    pub s_flag: BitState,
    pub rn: u8,
    pub rd: u8,
    pub shifter_operand: ShifterOperand,
}

impl<'a> From<DecodeData<'a>> for DataProcessingRegisterShift {
    fn from(data: DecodeData<'a>) -> Self {
        let opcode = DataProcessingInstruction::from((data.instruction.val >> 21) & 0b1111);
        let s_flag = BitState::from(data.instruction.val >> 20);
        let rn = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let shifter_operand = ShifterOperand::get_register_shift(data);

        Self {
            opcode,
            s_flag,
            rn,
            rd,
            shifter_operand,
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
    };

    use crate::{
        cpus::general::{
            instruction::encodings::encoding_fields::ShifterOperand,
            Instruction,
        },
        NintendoDS,
    };

    #[test]
    fn test_from() {
        let nds = NintendoDS::default();
        let data = {
            let instruction = Instruction {
                val: 0b0000_000_1111_1_1010_0101_0010_0_11_1_1001,
                ..Instruction::default()
            };
            DecodeData::new(instruction, &nds.arm7tdmi.registers)
        };

        let value = DataProcessingRegisterShift::from(data);

        let expected_value = DataProcessingRegisterShift {
            opcode: DataProcessingInstruction::MVN,
            s_flag: BitState::Set,
            rn: 0b1010,
            rd: 0b0101,
            shifter_operand: ShifterOperand {
                val: 0,
                shifter_carry_out: BitState::Unset,
            },
        };

        assert_eq!(value, expected_value);
    }
}
