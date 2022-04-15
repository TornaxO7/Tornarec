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
    BL,
    BIC,
    BKPT,
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
    LDRBT,
    LDRD,
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
    SMLAXY,
    SMLAL,
    SMLALXY,
    SMLAWY,
    SMULXY,
    SMULL,
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

    pub fn unknown_opcode(value: Word) -> ! {
        unreachable!("Unknown opcode: {:#034b}", value);
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
                _ => unreachable!(),
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
        }
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

#[cfg(test)]
mod tests {

    use super::ArmOpcode;

    #[test]
    fn test_adc() {
        let value = 0b0000_0010_1011_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::ADC, ArmOpcode::from(value));
    }

    #[test]
    fn test_add() {
        let value = 0b0000_0010_1001_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::ADD, ArmOpcode::from(value));
    }

    #[test]
    fn test_and() {
        let value = 0b0000_0010_0001_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::AND, ArmOpcode::from(value));
    }

    #[test]
    fn test_b() {
        let value = 0b0000_1010_1111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::B, ArmOpcode::from(value));
    }

    #[test]
    fn test_bl() {
        let value = 0b0000_1011_1111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::BL, ArmOpcode::from(value));
    }

    #[test]
    fn test_bic() {
        let value = 0b0000_0011_1101_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::BIC, ArmOpcode::from(value));
    }

    #[test]
    fn test_bkpt() {
        let value = 0b1110_0001_0010_1111_1111_1111_0111_1111;
        assert_eq!(ArmOpcode::BKPT, ArmOpcode::from(value));
    }

    #[test]
    fn test_blx1() {
        let value = 0b1111_1011_1111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::BLX, ArmOpcode::from(value));
    }

    #[test]
    fn test_blx2() {
        let value = 0b0000_0001_0010_1111_1111_1111_0011_1111;
        assert_eq!(ArmOpcode::BLX, ArmOpcode::from(value));
    }

    #[test]
    fn test_bx() {
        let value = 0b0000_0001_0010_1111_1111_1111_0001_1111;
        assert_eq!(ArmOpcode::BX, ArmOpcode::from(value));
    }

    #[test]
    fn test_cdp() {
        let value = 0b0000_1110_1111_1111_1111_1111_1110_1111;
        assert_eq!(ArmOpcode::CDP, ArmOpcode::from(value));
    }

    #[test]
    fn test_clz() {
        let value = 0b0000_0001_0110_1111_1111_1111_0001_1111;
        assert_eq!(ArmOpcode::CLZ, ArmOpcode::from(value));
    }

    #[test]
    fn test_cmn() {
        let value = 0b0000_0011_0111_1111_0000_1111_1111_1111;
        assert_eq!(ArmOpcode::CMN, ArmOpcode::from(value));
    }

    #[test]
    fn test_cmp() {
        let value = 0b0000_0011_0101_1111_0000_1111_1111_1111;
        assert_eq!(ArmOpcode::CMP, ArmOpcode::from(value));
    }

    #[test]
    fn test_eor() {
        let value = 0b0000_0010_0011_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::EOR, ArmOpcode::from(value));
    }

    #[test]
    fn test_ldc() {
        let value = 0b0000_1101_1111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::LDC, ArmOpcode::from(value));
    }

    #[test]
    fn test_ldc2() {
        let value = 0b1111_1101_1111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::LDC2, ArmOpcode::from(value));
    }

    #[test]
    fn test_ldm1() {
        let value = 0b0000_1001_1011_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::LDM, ArmOpcode::from(value));
    }

    #[test]
    fn test_ldm2() {
        let value = 0b0000_1001_1101_1111_0111_1111_1111_1111;
        assert_eq!(ArmOpcode::LDM, ArmOpcode::from(value));
    }

    #[test]
    fn test_ldm3() {
        let value = 0b0000_1001_1111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::LDM, ArmOpcode::from(value));
    }

    #[test]
    fn test_ldr() {
        let value = 0b0000_0111_1011_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::LDR, ArmOpcode::from(value));
    }

    #[test]
    fn test_ldrb() {
        let value = 0b0000_0111_1111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::LDRB, ArmOpcode::from(value));
    }

    #[test]
    fn test_ldrbt() {
        let value = 0b0000_0110_1111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::LDRBT, ArmOpcode::from(value));
    }

    #[test]
    fn test_ldrd() {
        let value = 0b0000_0001_1110_1111_1111_1111_1101_1111;
        assert_eq!(ArmOpcode::LDRD, ArmOpcode::from(value));
    }

    #[test]
    fn test_ldrh() {
        let value = 0b0000_0001_1111_1111_1111_1111_1011_1111;
        assert_eq!(ArmOpcode::LDRH, ArmOpcode::from(value));
    }

    #[test]
    fn test_ldrsb() {
        let value = 0b0000_0001_1111_1111_1111_1111_1101_1111;
        assert_eq!(ArmOpcode::LDRSB, ArmOpcode::from(value));
    }

    #[test]
    fn test_ldrsh() {
        let value = 0b0000_0001_1111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::LDRSH, ArmOpcode::from(value));
    }

    #[test]
    fn test_ldrt() {
        let value = 0b0000_0110_1011_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::LDRT, ArmOpcode::from(value));
    }

    #[test]
    fn test_mcr() {
        let value = 0b0000_1110_1110_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::MCR, ArmOpcode::from(value));
    }

    #[test]
    fn test_mcr2() {
        let value = 0b1111_1110_1110_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::MCR, ArmOpcode::from(value));
    }

    #[test]
    fn test_mcrr() {
        let value = 0b0000_1100_0100_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::MCRR, ArmOpcode::from(value));
    }

    #[test]
    fn test_mla() {
        let value = 0b0000_0000_0011_1111_1111_1111_1001_1111;
        assert_eq!(ArmOpcode::MLA, ArmOpcode::from(value));
    }

    #[test]
    fn test_mov() {
        let value = 0b0000_0011_1011_0000_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::MOV, ArmOpcode::from(value));
    }

    #[test]
    fn test_mrc() {
        let value = 0b0000_1110_1111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::MRC, ArmOpcode::from(value));
    }

    #[test]
    fn test_mrc2() {
        let value = 0b1111_1110_1111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::MRC, ArmOpcode::from(value));
    }

    #[test]
    fn test_mrrc() {
        let value = 0b0000_1100_0101_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::MRRC, ArmOpcode::from(value));
    }

    #[test]
    fn test_mrs() {
        let value = 0b0000_0001_0100_1111_1111_0000_0000_0000;
        assert_eq!(ArmOpcode::MRS, ArmOpcode::from(value));
    }

    #[test]
    fn test_msr_immediate() {
        let value = 0b0000_0011_0110_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::MSR, ArmOpcode::from(value));
    }

    #[test]
    fn test_msr_register() {
        let value = 0b0000_0001_0110_1111_1111_0000_0000_1111;
        assert_eq!(ArmOpcode::MSR, ArmOpcode::from(value));
    }

    #[test]
    fn test_mul() {
        let value = 0b0000_0000_0001_1111_0000_1111_1001_1111;
        assert_eq!(ArmOpcode::MUL, ArmOpcode::from(value));
    }

    #[test]
    fn test_mvn() {
        let value = 0b0000_0011_1111_0000_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::MVN, ArmOpcode::from(value));
    }

    #[test]
    fn test_orr() {
        let value = 0b0000_0011_1001_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::ORR, ArmOpcode::from(value));
    }

    #[test]
    fn test_pld() {
        let value = 0b1111_0111_1101_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::PLD, ArmOpcode::from(value));
    }

    #[test]
    fn test_qadd() {
        let value = 0b0000_0001_0000_1111_1111_0000_0101_1111;
        assert_eq!(ArmOpcode::QADD, ArmOpcode::from(value));
    }

    #[test]
    fn test_qdadd() {
        let value = 0b0000_0001_0100_1111_1111_0000_0101_1111;
        assert_eq!(ArmOpcode::QDADD, ArmOpcode::from(value));
    }

    #[test]
    fn test_qdsub() {
        let value = 0b0000_0001_0110_1111_1111_0000_0101_1111;
        assert_eq!(ArmOpcode::QDSUB, ArmOpcode::from(value));
    }

    #[test]
    fn test_qsub() {
        let value = 0b0000_0001_0010_1111_1111_0000_0101_1111;
        assert_eq!(ArmOpcode::QSUB, ArmOpcode::from(value));
    }

    #[test]
    fn test_rsb() {
        let value = 0b0000_0010_0111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::RSB, ArmOpcode::from(value));
    }

    #[test]
    fn test_rsc() {
        let value = 0b0000_0010_1111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::RSC, ArmOpcode::from(value));
    }

    #[test]
    fn test_sbc() {
        let value = 0b0000_0010_1101_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::SBC, ArmOpcode::from(value));
    }

    #[test]
    fn test_smla_xy() {
        let value = 0b0000_0001_0000_1111_1111_1111_1110_1111;
        assert_eq!(ArmOpcode::SMLAXY, ArmOpcode::from(value));
    }

    #[test]
    fn test_smlal() {
        let value = 0b0000_0000_1111_1111_1111_1111_1001_1111;
        assert_eq!(ArmOpcode::SMLAL, ArmOpcode::from(value));
    }

    #[test]
    fn test_smlalxy() {
        let value = 0b0000_0001_0100_1111_1111_1111_1110_1111;
        assert_eq!(ArmOpcode::SMLALXY, ArmOpcode::from(value));
    }

    #[test]
    fn test_smlawy() {
        let value = 0b0000_0001_0010_1111_1111_1111_1100_1111;
        assert_eq!(ArmOpcode::SMLAWY, ArmOpcode::from(value));
    }

    #[test]
    fn test_smulxy() {
        let value = 0b0000_0001_0110_1111_0000_1111_1110_1111;
        assert_eq!(ArmOpcode::SMLAXY, ArmOpcode::from(value));
    }

    #[test]
    fn test_smull() {
        let value = 0b0000_0000_1101_1111_1111_1111_1001_1111;
        assert_eq!(ArmOpcode::SMULL, ArmOpcode::from(value));
    }

    #[test]
    fn test_smulwy() {
        let value = 0b0000_0001_0010_1111_0000_1111_1110_1111;
        assert_eq!(ArmOpcode::SMULWY, ArmOpcode::from(value));
    }

    #[test]
    fn test_stc() {
        let value = 0b0000_1101_1110_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::STC, ArmOpcode::from(value));
    }

    #[test]
    fn test_stc2() {
        let value = 0b1111_1101_1110_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::STC, ArmOpcode::from(value));
    }

    #[test]
    fn test_stm1() {
        let value = 0b0000_1001_1010_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::STM, ArmOpcode::from(value));
    }

    #[test]
    fn test_stm2() {
        let value = 0b0000_1001_1100_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::STM, ArmOpcode::from(value));
    }

    #[test]
    fn test_str() {
        let value = 0b0000_0111_1010_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::STR, ArmOpcode::from(value));
    }

    #[test]
    fn test_strb() {
        let value = 0b0000_0111_1110_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::STRB, ArmOpcode::from(value));
    }

    #[test]
    fn test_strbt() {
        let value = 0b0000_0110_1110_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::STRBT, ArmOpcode::from(value));
    }

    #[test]
    fn test_strd() {
        let value = 0b0000_0001_1110_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::STRD, ArmOpcode::from(value));
    }

    #[test]
    fn test_strh() {
        let value = 0b0000_0001_1110_1111_1111_1111_1011_1111;
        assert_eq!(ArmOpcode::STRH, ArmOpcode::from(value));
    }

    #[test]
    fn test_strt() {
        let value = 0b0000_0110_1010_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::STRT, ArmOpcode::from(value));
    }

    #[test]
    fn test_sub() {
        let value = 0b0000_0010_0101_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::SUB, ArmOpcode::from(value));
    }

    #[test]
    fn test_swi() {
        let value = 0b0000_1111_1111_1111_1111_1111_1111_1111;
        assert_eq!(ArmOpcode::SWI, ArmOpcode::from(value));
    }

    #[test]
    fn test_swp() {
        let value = 0b0000_0001_0000_1111_1111_0000_1001_1111;
        assert_eq!(ArmOpcode::SWP, ArmOpcode::from(value));
    }

    #[test]
    fn test_swpb() {
        let value = 0b0000_0001_0100_1111_1111_0000_1001_1111;
        assert_eq!(ArmOpcode::SWPB, ArmOpcode::from(value));
    }

    #[test]
    fn test_teq() {
        let value = 0b0000_0011_0011_1111_0000_1111_1111_1111;
        assert_eq!(ArmOpcode::TEQ, ArmOpcode::from(value));
    }

    #[test]
    fn test_tst() {
        let value = 0b0000_0011_0001_1111_0000_1111_1111_1111;
        assert_eq!(ArmOpcode::TST, ArmOpcode::from(value));
    }

    #[test]
    fn test_umlal() {
        let value = 0b0000_0000_1011_1111_1111_1111_1001_1111;
        assert_eq!(ArmOpcode::UMLAL, ArmOpcode::from(value));
    }

    #[test]
    fn test_umull() {
        let value = 0b0000_0000_1001_1111_1111_1111_1001_1111;
        assert_eq!(ArmOpcode::UMULL, ArmOpcode::from(value));
    }
}
