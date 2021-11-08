use crate::cpus::general::{
    instruction::{
        decode::DecodeData,
        encodings::encoding_fields::{
            DataProcessingInstruction,
            ShifterOperand,
        },
    },
    register::RegisterName,
    BitState,
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingData {
    pub opcode: DataProcessingInstruction,
    pub s_flag: BitState,
    pub rn_reg: RegisterName,
    pub rd_reg: RegisterName,
    pub shifter_operand: ShifterOperand,
}

impl<'a> From<DecodeData<'a>> for DataProcessingData {
    fn from(data: DecodeData<'a>) -> Self {
        let opcode = DataProcessingInstruction::from(data.instruction.val >> 21);
        let s_flag = BitState::from(data.instruction.val >> 20);
        let rn_reg = RegisterName::from((data.instruction.val >> 16) & 0b1111);
        let rd_reg = RegisterName::from((data.instruction.val >> 12) & 0b1111);

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
            rn_reg,
            rd_reg,
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
        RegisterName,
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
            rn_reg: RegisterName::R12,
            rd_reg: RegisterName::R3,
            shifter_operand: ShifterOperand::get_immediate(data),
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
