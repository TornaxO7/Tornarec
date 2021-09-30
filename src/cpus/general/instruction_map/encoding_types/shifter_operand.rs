use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShifterOperand {
    Immediate(u32),
}

impl From<u32> for ShifterOperand {
    fn from(val: u32) -> Self {
        let rotate_imm = (val >> 8) & 0b1111;
        let immed_8 = val & 0b1111_1111;
        Self::Immediate(immed_8 << rotate_imm)
    }
}
