pub mod error;
pub mod operand;
pub mod msr_operand;

pub use error::CpsrAccessError;
pub use operand::CpsrAccessOperand;

use crate::cpus::general::{
    bit_state::BitState,
    instruction_map::{
        InstructionMapTrait,
        cpsr_access::msr_operand::MsrOperand,
    },
    instruction::Instruction,
    register::types::RegisterIndex,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpsrAccess(Instruction);


impl InstructionMapTrait for CpsrAccess {
    type Operand = CpsrAccessOperand;

    fn is_matching(instruction: &Instruction) -> bool {
        CpsrAccessOperand::is_msr(instruction)
            || CpsrAccessOperand::is_mrs(instruction)
    }

    fn get_operand(&self) -> Self::Operand {
        let instruction_val = self.0.get_value_as_u32();

        if CpsrAccessOperand::is_msr(&self.0) {
            let r_flag = BitState::from((instruction_val >> 22) & 0b1);
            let rd = RegisterIndex::from((instruction_val >> 12) & 0b1111);

            if (instruction_val >> 16) & 0b1111 != 0b1111 {
                panic!("{}", CpsrAccessError::SBOConflict(16, 19, instruction_val));
            }
            if instruction_val & 0b111_1111_1111 != 0b000_0000_0000 {
                panic!("{}", CpsrAccessError::SBZConflict(0, 11, instruction_val));
            }

            CpsrAccessOperand::MRS {
                r_flag,
                rd,
            }
        } else if CpsrAccessOperand::is_mrs(&self.0) {
            CpsrAccessOperand::MSR(MsrOperand::from(&self.0))
        } else {
            unreachable!("{}", CpsrAccessError::UnknownOperand(instruction_val));
        }
    }
}
