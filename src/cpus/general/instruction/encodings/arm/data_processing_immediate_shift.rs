use crate::cpus::general::{
    bit_state::BitState,
    instruction::{
        decode::DecodeData,
        encodings::encoding_fields::{
            DataProcessingInstruction,
            RegisterOrValue,
            ShifterOperand,
        },
    },
    register::{
        NormalizedRegister,
        RegisterName,
    },
};

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProcessingImmediateShift {
    pub opcode: DataProcessingInstruction,
    pub s_flag: BitState,
    pub rn: RegisterOrValue,
    pub rd: NormalizedRegister,
    pub shifter_operand: ShifterOperand,
}

impl From<DecodeData> for DataProcessingImmediateShift {
    fn from(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();
        let next_instruction_val = data.next_instruction.get_value_as_u32();

        let opcode = DataProcessingInstruction::from((instruction_val >> 21) & 0b1111);
        let s_flag = BitState::from(instruction_val >> 20);

        let rn = (instruction_val >> 16) & 0b1111;
        let rn = if NormalizedRegister::from(rn) == RegisterName::Pc {
            RegisterOrValue::Value(next_instruction_val)
        } else {
            RegisterOrValue::Register(NormalizedRegister::from(rn))
        };

        let rd = NormalizedRegister::from((instruction_val >> 12) & 0b1111);
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
        NormalizedRegister,
        RegisterName,
    };

    use crate::{
        cpus::general::Instruction,
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
