use crate::ram::Word;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmOpcode {
    ADC,
    ADD,
    AND,
    B,
    BIC,
    BKPT,
    BL,
    BLX,
    BX,
    CDP,
    CDP2,
    CLZ,
    CMN,
    CMP,
    EOR,
    LDC,
    LDC2,
    LDM,
    LDR,
    LDRB,
    LDRD,
    LDRBT,
    LDRH,
    LDRSB,
    LDRSH,
    LDRT,
    MCR,
    MCR2,
    MCRR,
    MLA,
    MOV,
    MRC,
    MRC2,
    MRRC,
    MRS,
    MSR,
    MUL,
    MVN,
    ORR,
    PLD,
    QADD,
    QDADD,
    QDSUB,
    QSUB,
    RSB,
    RSC,
    SBC,
    SMLAL,
    SMLAXY,
    SMLALXY,
    SMLAWY,
    SMULL,
    SMULXY,
    STC,
    STC2,
    STM,
    STR,
    STRB,
    STRBT,
    STRD,
    STRH,
    STRT,
    SUB,
    SWI,
    SWP,
    SWPB,
    TEQ,
    TST,
    UMLAL,
    UMULL,
    NOOP,
}

impl ArmOpcode {
    pub fn get_data_processing_opcode(value: Word) -> Self {
        let opcode = (value >> 21) & 0b1111;
        match opcode {
            0b0000 => ArmOpcode::AND,
            0b0001 => ArmOpcode::EOR,
            0b0010 => ArmOpcode::SUB,
            0b0011 => ArmOpcode::RSB,
            0b0100 => ArmOpcode::ADD,
            0b0101 => ArmOpcode::ADC,
            0b0110 => ArmOpcode::SBC,
            0b0111 => ArmOpcode::RSC,
            0b1000 => ArmOpcode::TST,
            0b1001 => ArmOpcode::TEQ,
            0b1010 => ArmOpcode::CMP,
            0b1011 => ArmOpcode::CMN,
            0b1100 => ArmOpcode::ORR,
            0b1101 => ArmOpcode::MOV,
            0b1110 => ArmOpcode::BIC,
            0b1111 => ArmOpcode::MVN,
            _ => unreachable!("Non-Dataprocessing-Operand: {}", opcode),
        }
    }

    pub fn get_multiply(value: Word) -> Self {
        let bit21_20 = (value >> 20) & 0b11;
        match bit21_20 {
            0b00 => Self::MUL,
            0b01 => Self::MULS,
            0b10 => Self::MLA,
            0b11 => Self::MLAS,
            _ => unreachable!(),
        }
    }

    pub fn get_multiply_long(value: Word) -> Self {
        let bit22_20 = (value >> 20) & 0b111;
        match bit22_20 {
            0b000 => Self::SMULL,
            0b001 => Self::SMULLS,
            0b010 => Self::SMLAL,
            0b011 => Self::SMLALS,
            0b100 => Self::UMULL,
            0b101 => Self::UMULLS,
            0b110 => Self::UMLAL,
            0b111 => Self::UMLALS,
            _ => unreachable!(),
        }
    }
}
