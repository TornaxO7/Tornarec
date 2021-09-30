use core::convert::From;

use crate::cpus::general::instruction_map::encoding_types::field::{
    RotationBy,
    RotationDirection,
    RotateImm,
    Immed8,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShifterOperand {
    Immediate(RotateImm, Immed8),
    Register(u32),
    LogicalShift(u32, RotationBy, RotationDirection),
    ArithmeticShift(u32, RotationBy, RotationDirection),
    ArithmeticShiftRightByImmediate(u32),
    ArithmeticShiftRightByRegister(u32),
    RotateRightByImmediate(u32),
    RotateRightByRegister(u32),
    RotateRightWithExtend(u32),
}

impl From<u32> for ShifterOperand {
    fn from(val: u32) -> Self {
        let rotate_imm = RotateImm::from((val >> 8) & 0b1111);
        let immed_8 = Immed8::from(val);
        Self::Immediate(rotate_imm, immed_8)
    }
}
