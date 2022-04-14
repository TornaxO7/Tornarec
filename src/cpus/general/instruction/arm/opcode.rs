use crate::ram::Word;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmOpcode {
    ADC,
    ADD,
    AND,
    B,
    BL,
    BLX,
    BIC,
    BKPT,
    BLX1,
    BLX2,
    BX,
    BXJ,
    CDP,
    CLZ,
    CMN,
    CMP,
    CPS,
    CPY,
    EOR,
    LDC,
    LDM1,
    LDM2,
    LDM3,
    LDR,
    LDRB,
    LDRBT,
    LDRD,
    LDREX,
    LDRH,
    LDRSB,
    LDRSH,
    LDRT,
    MCR,
    MCRR,
    MLA,
    MLAS,
    MOV,
    MRC,
    MRRC,
    MRS,
    MSR,
    MUL,
    MULS,
    MVN,
    ORR,
    PKHBT,
    PKHTB,
    PLD,
    QADD,
    QDADD,
    QDSUB,
    QSUB,
    QSUB106,
    REV,
    REV16,
    REVSH,
    RFE,
    RSB,
    RSC,
    SBC,
    SEL,
    SMLAL,
    SMLALS,
    SMLAXY,
    SMLALXY,
    SMLALD,
    SMLAWY,
    SMLSD,
    SMLSLD,
    SMMLA,
    SMMLS,
    SMMUL,
    SMUAD,
    SMULXY,
    SMULL,
    SMULLS,
    SMULWY,
    SMUSD,
    SRS,
    SSAT,
    SSAT16,
    STC,
    STM1,
    SMT2,
    STR,
    STRB,
    STRBT,
    STRD,
    STREX,
    STRH,
    STRT,
    SUB,
    SWI,
    SWP,
    SWPB,
    TEQ,
    TST,
    UHSUB16,
    UMAAL,
    UMLAL,
    UMLALS,
    UMULL,
    UMULLS,
    USADA8,
    USAT,
    USAT16,
    NOOP,
}

impl ArmOpcode {
    pub fn get_data_processing_operand(value: Word) -> Self {
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
        let bit27_22 = (value >> 22) & 0b1111_11;
        let bit7_4 = (value >> 4) & 0b111;

        if bit27_22 != 0 && bit7_4 != 0b1001 {
            unreachable!("[ArmOpcode] Unknown multiply opcode: {:#034b}", value);
        }

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
        let bit27_23 = (value >> 23) & 0b1111_1;
        let bit7_4 = (value >> 4) & 0b1111;

        if bit27_23 != 0b0000_1 || bit7_4 != 0b1001 {
            todo!("[Long multiply] Unknown long multiply: {:#034b}", value);
        }

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
