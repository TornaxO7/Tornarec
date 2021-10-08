pub mod operand;
pub mod addressing_mode;
pub mod error;

pub use operand::NormalOperand;
pub use addressing_mode::AddressingMode;

use std::convert::From;

use crate::cpus::general::instruction::Instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Normal {
    operand: NormalOperand,
    rn: u8,
    rd: u8,
    addressing_mode: AddressingMode,
}

impl From<&Instruction> for Normal {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let operand = NormalOperand::from(instruction);
        let rn = (instruction_val >> 16) & 0b1111;
        let rd = (instruction_val >> 12) & 0b1111;
        let addressing_mode = AddressingMode::from(instruction);

        Self{operand, rn, rd, addressing_mode}
    }
}
