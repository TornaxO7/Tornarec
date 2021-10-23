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
        register::{
            NormalizedRegister,
            RegisterName,
        },
    },
    ram::data_types::DataTypeSize,
};

use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingImmediateShift {
    pub opcode: DataProcessingInstruction,
    pub s_flag: BitState,
    pub rn: u32,
    pub rd: u8,
    pub shifter_operand: ShifterOperand,
}

impl<'a> From<DecodeData<'a>> for DataProcessingImmediateShift {
    fn from(data: DecodeData<'a>) -> Self {
        let opcode = DataProcessingInstruction::from((data.instruction.val >> 21) & 0b1111);
        let s_flag = BitState::from(data.instruction.val >> 20);

        let rn = {
            let rn = (data.instruction.val >> 16) & 0b1111;

            if NormalizedRegister::from(rn) == RegisterName::Pc {
                let value = data.instruction.address + DataTypeSize::Byte;
                value.get_as_u32()
            } else {
                rn
            }
        };

        let rd = u8::try_from((data.instruction.val >> 12) & 0b1111).unwrap();
        let shifter_operand = ShifterOperand::get_immediate_shift(data);

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
        DataProcessingImmediateShift,
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
    fn test_from() {
        let nds = NintendoDS::default();
        let instruction = Instruction {
            val: 0b0000_000_1010_1_1010_0101_11100_10_0_1001,
            ..Instruction::default()
        };
        let data = DecodeData::new(instruction, &nds.arm7tdmi.registers);

        let value = DataProcessingImmediateShift::from(data.clone());

        let expected_value = DataProcessingImmediateShift {
            opcode: DataProcessingInstruction::CMP,
            s_flag: BitState::Set,
            rn: 0b1010,
            rd: 0b0101,
            shifter_operand: ShifterOperand::get_immediate_shift(data),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
