use std::convert::{
    From,
    TryFrom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataProcessingInstruction {
    AND,
    EOR,
    SUB,
    RSB,
    ADD,
    ADC,
    SBC,
    RSC,
    TST,
    TEQ,
    CMP,
    CMN,
    ORR,
    MOV,
    BIC,
    MVN,
}

impl From<u8> for DataProcessingInstruction {
    fn from(opcode: u8) -> Self {
        match opcode & 0b1111 {
            0b0000 => Self::AND,
            0b0001 => Self::EOR,
            0b0010 => Self::SUB,
            0b0011 => Self::RSB,
            0b0100 => Self::ADD,
            0b0101 => Self::ADC,
            0b0110 => Self::SBC,
            0b0111 => Self::RSC,
            0b1000 => Self::TST,
            0b1001 => Self::TEQ,
            0b1010 => Self::CMP,
            0b1011 => Self::CMN,
            0b1100 => Self::ORR,
            0b1101 => Self::MOV,
            0b1110 => Self::BIC,
            0b1111 => Self::MVN,
            _ => unreachable!("[DataProcessingInstruction Error]: This shouldn't happen..."),
        }
    }
}

impl From<u32> for DataProcessingInstruction {
    fn from(num: u32) -> Self {
        Self::from(u8::try_from(num & 0b1111).unwrap())
    }
}
