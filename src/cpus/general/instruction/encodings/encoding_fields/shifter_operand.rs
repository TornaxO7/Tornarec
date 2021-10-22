use crate::cpus::general::{
    BitState,
    instruction::encodings::arm::DataProcessingImmediateShift,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShifterOperand {
    shifter_operand: u16,
    shifter_carry_out: BitState,
}

// impl From<&DataProcessingImmediateShift> for ShifterOperand {
//     fn from(data: &DataProcessingImmediateShift) -> Self {
//     }
// }
