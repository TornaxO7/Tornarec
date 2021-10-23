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
pub struct DataProcessingImmediate {
    pub opcode: DataProcessingInstruction,
    pub s_flag: BitState,
    pub rn: RegisterOrValue,
    pub rd: NormalizedRegister,
    pub shifter_operand: ShifterOperand,
}

impl From<DecodeData> for DataProcessingImmediate {
    fn from(data: DecodeData) -> Self {
        let instruction_val = data.instruction.get_value_as_u32();

        let opcode = DataProcessingInstruction::from((instruction_val >> 21) & 0b1111);
        let s_flag = BitState::from(instruction_val >> 20);

        let rn = (instruction_val >> 16) & 0b1111;
        let rn = if NormalizedRegister::from(rn) == RegisterName::Pc {
            RegisterOrValue::Value(instruction_val >> 20)
        } else {
            RegisterOrValue::Register(NormalizedRegister::from(rn))
        };

        let rd = NormalizedRegister::from((instruction_val >> 12) & 0b1111);
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
    fn from() {
        let nds = NintendoDS::default();
        let instruction = Instruction::from(0b0000_001_1111_1_1100_0011_1010_0011_1011);
        let data = DecodeData::new(&nds.arm7tdmi.registers, &nds.ram, &instruction);

        let value = DataProcessingImmediate::from(data);

        let expected_value = DataProcessingImmediate {
            opcode: DataProcessingInstruction::MVN,
            s_flag: BitState::Set,
            rn: NormalizedRegister::from(RegisterName::R12),
            rd: NormalizedRegister::from(RegisterName::R3),
            rotate: 0b1010,
            immediate: 0b0011_1011,
        };

        assert_eq!(
            value, expected_value,
            "{:#?}, {:#?}",
            &value, &expected_value
        );
    }
}
