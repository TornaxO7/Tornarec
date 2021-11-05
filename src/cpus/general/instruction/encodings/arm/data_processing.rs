use crate::cpus::general::{
    instruction::{
        decode::DecodeData,
        encodings::encoding_fields::{
            DataProcessingInstruction,
            ShifterOperand,
        },
    },
    BitState,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingData {
    pub opcode: DataProcessingInstruction,
    pub s_flag: BitState,
    pub rn: u8,
    pub rd: u8,
    pub shifter_operand: ShifterOperand,
}

impl<'a> From<DecodeData<'a>> for DataProcessingData {
    fn from(data: DecodeData<'a>) -> Self {
        let opcode = DataProcessingInstruction::from(data.instruction.val >> 21);
        let s_flag = BitState::from(data.instruction.val >> 20);
        let rn = u8::try_from((data.instruction.val >> 16) & 0b1111).unwrap();
        let rd = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();

        let shifter_operand = {
            let is_immediate = (data.instruction.val >> 25) & 0b1 == 1;

            if is_immediate {
                ShifterOperand::get_immediate(data)
            } else if (data.instruction.val >> 7) & 0b1 == 0 && (data.instruction.val >> 4) & 1 == 1
            {
                ShifterOperand::get_immediate_shift(data)
            } else {
                ShifterOperand::get_register_shift(data)
            }
        };

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
    use crate::{
        cpus::general::{
            instruction::decode::DecodeData,
            BitState,
            Instruction,
        },
        NintendoDS,
    };

    use super::{
        DataProcessingData,
        DataProcessingInstruction,
        ShifterOperand,
    };

    #[test]
    fn immediate() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b0000_001_1111_1_1100_0011_1010_0011_1011,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = DataProcessingData::from(data.clone());

        let expected_value = DataProcessingData {
            opcode: DataProcessingInstruction::MVN,
            s_flag: BitState::Set,
            rn: 0b1100,
            rd: 0b0011,
            shifter_operand: ShifterOperand::get_immediate(data),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
