use crate::ram::Word;

use super::BitState;

mod opcode000;
mod opcode001;
mod opcode010;
mod opcode011;
mod opcode100;
mod opcode101;
mod opcode110;
mod opcode111;

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
    SMULWY,
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
    pub fn get_data_processing(value: Word) -> Self {
        let bit24_21 = (value >> 21) & 0b1111;

        match bit24_21 {
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
            _ => unreachable!("Unknown data-processing instruction: {:#034b}", value),
        }
    }
}

impl From<Word> for ArmOpcode {
    fn from(value: Word) -> Self {
        let bit32_28 = (value >> 28) & 0b1111;
        if bit32_28 == 0b1111 {
            get_unconditional_instruction(value)
        } else {
            let bit27_25 = (value >> 25) & 0b111;

            match bit27_25 {
                0b000 => opcode000::handle(value),
                0b001 => opcode001::handle(value),
                0b010 => opcode010::handle(value),
                0b011 => opcode011::handle(value),
                0b100 => opcode100::handle(value),
                0b101 => opcode101::handle(value),
                0b110 => opcode110::handle(value),
                0b111 => opcode111::handle(value),
            }
        }
    }
}

fn get_unconditional_instruction(value: Word) -> ArmOpcode {
    let bit27_26 = (value >> 26) & 0b11;
    let bit25 = (value >> 25) & 0b1;

    match (bit27_26, bit25) {
        (0b01, _) => ArmOpcode::PLD,
        (0b10, 1) => ArmOpcode::BLX,
        (0b11, 0) => {
            let l_flag = BitState::from(((value >> 20) & 0b1) != 0);
            match l_flag {
                false => ArmOpcode::STC2,
                true => ArmOpcode::LDC2,
            }
        },
        (0b11, 1) => {
            let bit20 = (value >> 20) & 0b1;
            let bit4 = (value >> 4) & 0b1;

            match (bit20, bit4) {
                (_, 0) => ArmOpcode::CDP2,
                (0, 1) => ArmOpcode::MCR2,
                (1, 1) => ArmOpcode::MRC2,
                (_, _) => unreachable!(),
            }
        }
        (_, _) => unreachable!(),
    }
}
