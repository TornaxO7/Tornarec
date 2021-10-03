pub mod operand;

use crate::cpus::general::{
    instruction::Instruction,
    instruction_map::{
        InstructionMapTrait,
        multiply::operand::MultiplyOperand,
        encoding_types::field::immed_8::Immed8,
    },
    bit_state::BitState,
    register::types::RegisterIndex,
};

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Multiply(Instruction);

impl InstructionMapTrait for Multiply {

    type Operand = MultiplyOperand;

    fn is_matching(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();

        if (instruction_val >> 4) & 0b111 != 0b1001 {
            return false;
        }

        [
            MultiplyOperand::MLA_CODE,
            MultiplyOperand::MUL_CODE,
            MultiplyOperand::SMLAL_CODE,
            MultiplyOperand::SMULL_CODE,
            MultiplyOperand::UMLAL_CODE,
            MultiplyOperand::UMULL_CODE,
        ].iter()
            .any(|operand_code| *operand_code == (instruction_val >> 21) & 0b111_1111)

    }

    fn get_operand(&self) -> Self::Operand {
        let instruction_val = self.0.get_value_as_u32();
        let operand_code = (instruction_val >> 21) & 0b111_1111;

        let s_flag = BitState::from((instruction_val >> 20) & 0b1);
        let rm = RegisterIndex::from(instruction_val & 0b1111);
        let rs = RegisterIndex::from((instruction_val >> 8) & 0b1111);

        match operand_code {
            MultiplyOperand::MLA_CODE => {
                let rd = RegisterIndex::from((instruction_val >> 16) & 0b1111);
                let rn = RegisterIndex::from((instruction_val >> 12) & 0b1111);

                MultiplyOperand::MLA {
                    s_flag,
                    rd,
                    rn,
                    rs,
                    rm,
                }
            },
            MultiplyOperand::MUL_CODE => {
                if ((instruction_val >> 12) & 0b1111) != 0b0000 {
                    panic!("[MULTIPLY ERROR]: MUL-Instruction doesn't have zero bits from 12:15: {:b}!", instruction_val);
                }

                let rd = RegisterIndex::from((instruction_val >> 16) & 0b1111);

                MultiplyOperand::MUL {
                    s_flag,
                    rd,
                    rs,
                    rm,
                }
            },
            MultiplyOperand::SMLAL_CODE => {
                let rdhi = Immed8::from((instruction_val >> 16) & 0b1111);
                let rdlo = Immed8::from((instruction_val >> 12) & 0b1111);

                MultiplyOperand::SMLAL {
                    s_flag,
                    rdhi,
                    rdlo,
                    rs,
                    rm,
                }
            },
            MultiplyOperand::SMULL_CODE => {
                let rdhi = Immed8::from((instruction_val >> 16) & 0b1111);
                let rdlo = Immed8::from((instruction_val >> 12) & 0b1111);

                MultiplyOperand::SMULL {
                    s_flag,
                    rdhi,
                    rdlo,
                    rs,
                    rm,
                }

            },
            MultiplyOperand::UMLAL_CODE => {
                let rdhi = Immed8::from((instruction_val >> 16) & 0b1111);
                let rdlo = Immed8::from((instruction_val >> 12) & 0b1111);

                MultiplyOperand::UMLAL {
                    s_flag,
                    rdhi,
                    rdlo,
                    rs,
                    rm,
                }
            },
            MultiplyOperand::UMULL_CODE => {
                let rdhi = Immed8::from((instruction_val >> 16) & 0b1111);
                let rdlo = Immed8::from((instruction_val >> 12) & 0b1111);

                MultiplyOperand::UMULL {
                    s_flag,
                    rdhi,
                    rdlo,
                    rs,
                    rm,
                }
            },

            _other => unreachable!("[MULTIPLY ERROR]: Unknown multiply operand: {:b}.", _other),
        }
    }
}

impl From<&Instruction> for Multiply {
    fn from(instruction: &Instruction) -> Self {
        Self(instruction.clone())
    }
}
