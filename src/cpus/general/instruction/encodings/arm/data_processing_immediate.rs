use crate::{
    cpus::general::{
        bit_state::BitState,
        instruction::{
            decode::DecodeData,
            encodings::encoding_fields::{
                DataProcessingInstruction,
                ShifterOperand,
            },
        },
        register::RegisterName,
    },
    ram::data_types::DataTypeSize,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingImmediate {
    pub opcode: DataProcessingInstruction,
    pub s_flag: BitState,
    pub rn: u32,
    pub rd: u8,
    pub shifter_operand: ShifterOperand,
}

impl<'a> From<DecodeData<'a>> for DataProcessingImmediate {
    fn from(data: DecodeData<'a>) -> Self {
        let opcode = DataProcessingInstruction::from((data.instruction.val >> 21) & 0b1111);
        let s_flag = BitState::from(data.instruction.val >> 20);

        let rn = {
            let rn = (data.instruction.val >> 16) & 0b1111;
            if RegisterName::from(rn) == RegisterName::R15 {
                let value = data.instruction.address + DataTypeSize::Byte;
                value.get_as_u32()
            } else {
                rn
            }
        };

        let rd = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let shifter_operand = ShifterOperand::get_immediate(data);

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
        DataProcessingImmediate,
        DataProcessingInstruction,
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
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b0000_001_1111_1_1100_0011_1010_0011_1011,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = DataProcessingImmediate::from(data.clone());

        let expected_value = DataProcessingImmediate {
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
