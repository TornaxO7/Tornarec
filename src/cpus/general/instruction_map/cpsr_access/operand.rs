use crate::cpus::general::{
    BitState,
    instruction::Instruction,
    instruction_map::cpsr_access::msr_operand::MsrOperand,
    register::types::RegisterIndex
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpsrAccessOperand {
    MRS {
        r_flag: BitState,
        rd: RegisterIndex,
    },
    MSR(MsrOperand),
}

impl CpsrAccessOperand {
    pub fn is_msr(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();

        (instruction_val >> 23) & 0b11111 == 0b00110
            && (instruction_val >> 20) & 0b11 == 0b10
    }

    pub fn is_mrs(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();
        
        (instruction_val >> 23) & 0b11111 == 0b00010
            && (instruction_val >> 20) & 0b11 == 0b00
    }
}
