use crate::{
    cpus::general::OperatingMode,
    ram::Word,
};

use std::convert::TryFrom;

use super::{
    encoding_fields::{
        AddressingMode1Offset,
        AddressingMode2Offset,
        AddressingMode3Offset,
        AddressingMode4Offset,
        AddressingMode5Offset,
        MSRType,
        RegisterList,
    },
    BitState,
    CPNum,
    CPOpcode,
    CPRegister,
    Register,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmOperand {
    Branch(u32),

    // Multiply stuff
    NormalMultiply {
        s: BitState,
        rd: Register,
        rs: Register,
        rm: Register,
    },
    LongMultiply {
        rdhi: u8,
        rdlo: u8,
        rs: Register,
        rm: Register,
    },
    HalfwordMultiply {
        rd: Register,
        rs: Register,
        y: BitState,
        x: BitState,
        rm: Register,
    },
    WordHalfwordMultiply {
        rd: Register,
        rs: Register,
        rm: Register,
        y: BitState,
    },
    MostSignificantWordMultiply {
        rd: Register,
        rs: Register,
        rm: Register,
        r: BitState,
    },
    DualHalfwordMultiply {
        rd: Register,
        rs: Register,
        x: BitState,
        rm: Register,
    },

    CountLeadingZeros {
        rd: Register,
        rm: Register,
    },

    // Status register stuff
    MRS {
        r: BitState,
        rd: Register,
    },

    MSR {
        r: BitState,
        field_mask: u8,
        msr_type: MSRType,
    },

    CPS {
        imod: u8,
        mmod: BitState,
        a: BitState,
        i: BitState,
        f: BitState,
        mode: OperatingMode,
    },

    // Address modes stuff
    AddressingMode1 {
        s: BitState,
        rn: Register,
        rd: Register,
        offset: AddressingMode1Offset,
    },
    AddressingMode2 {
        p: BitState,
        u: BitState,
        b: BitState,
        w: BitState,
        l: BitState,
        rn: Register,
        rd: Register,
        offset: AddressingMode2Offset,
    },
    AddressingMode3 {
        p: BitState,
        u: BitState,
        w: BitState,
        l: BitState,
        rn: Register,
        rd: Register,
        offset: AddressingMode3Offset,
    },
    AddressingMode4 {
        s: BitState,
        w: BitState,
        l: BitState,
        rn: Register,
        register_list: RegisterList,
        offset: AddressingMode4Offset,
    },
    AddressingMode5 {
        u: BitState,
        n: BitState,
        l: BitState,
        rn: Register,
        crd: CPRegister,
        cp_num: CPNum,
        offset_option: u8,
        offset_type: AddressingMode5Offset,
    },

    Semaphore {
        rn: Register,
        rd: Register,
        rm: Register,
    },

    SWI(u32),
    BKPT {
        immed1: u16,
        immed2: u8,
    },

    CPD {
        opcode1: CPOpcode,
        crn: CPRegister,
        crd: CPRegister,
        cp_num: CPNum,
        opcode2: CPOpcode,
        crm: CPRegister,
    },

    MCR {
        opcode1: CPOpcode,
        crn: CPRegister,
        rd: Register,
        cp_num: CPNum,
        opcode2: CPOpcode,
        crm: CPRegister,
    },

    MCRR {
        rn: Register,
        rd: Register,
        cp_num: CPNum,
        opcode: CPOpcode,
        crm: CPRegister,
    },

    MRC {
        opcode1: CPOpcode,
        crn: CPRegister,
        rd: Register,
        cp_num: CPNum,
        opcode2: CPOpcode,
        crm: CPRegister,
    },

    MRRC {
        rn: Register,
        rd: Register,
        cp_num: CPNum,
        opcode: u8,
        crm: CPRegister,
    },

    NOOP,
}

impl ArmOperand {
    pub fn get_branch(value: Word) -> Self {
        let immed = value & 0b1111_1111_1111_1111_1111_1111;
        Self::Branch(immed)
    }

    pub fn get_normal_multiply(value: Word) -> Self {
        let s = BitState::from(((value >> 20) & 0b1) != 0);
        let rd = Register::try_from((value >> 16) & 0b1111).unwrap();
        let sbz = (value >> 12) & 0b1111;
        let rs = Register::try_from((value >> 8) & 0b1111).unwrap();
        let rm = Register::try_from(value & 0b1111).unwrap();

        if sbz != 0 {
            todo!("SBZ, see A4.1.40 (page 230)");
        }

        Self::NormalMultiply { s, rd, rs, rm }
    }

    pub fn get_long_multiply(value: Word) -> Self {
        let rdhi = u8::try_from((value >> 16) & 0b1111).unwrap();
        let rdlo = u8::try_from((value >> 12) & 0b1111).unwrap();
        let rs = Register::try_from((value >> 8) & 0b1111).unwrap();
        let rm = Register::try_from(value & 0b1111).unwrap();

        Self::LongMultiply { rdhi, rdlo, rs, rm }
    }

    pub fn get_halfword_multiply(value: Word) -> Self {
        let rd = Register::try_from((value >> 16) & 0b1111).unwrap();
        let sbz = (value >> 12) & 0b1111;
        let rs = Register::try_from((value >> 8) & 0b1111).unwrap();
        let y = BitState::from(((value >> 6) & 0b1) != 0);
        let x = BitState::from(((value >> 5) & 0b1) != 0);
        let rm = Register::try_from(value & 0b1111).unwrap();

        if sbz != 0 {
            todo!("[SBZ] A4.1.86 (page 316)");
        }

        Self::HalfwordMultiply { rd, rs, y, x, rm }
    }

    pub fn get_word_halfword_multiply(value: Word) -> Self {
        let rd = Register::try_from((value >> 16) & 0b1111).unwrap();
        let sbz = (value >> 12) & 0b1111;
        let rs = Register::try_from((value >> 8) & 0b1111).unwrap();
        let y = BitState::from(((value >> 6) & 0b1) != 0);
        let rm = Register::try_from(value & 0b1111).unwrap();

        if sbz != 0 {
            todo!("[SBZ] A4.1.88 (page 320)");
        }

        Self::WordHalfwordMultiply { rd, rs, y, rm }
    }

    pub fn get_most_significant_word_multiply(value: Word) -> Self {
        let rd = Register::try_from((value >> 16) & 0b1111).unwrap();
        let rs = Register::try_from((value >> 8) & 0b1111).unwrap();
        let r = BitState::from(((value >> 5) & 0b1) != 0);
        let rm = Register::try_from(value & 0b1111).unwrap();

        Self::MostSignificantWordMultiply { rd, rs, r, rm }
    }

    pub fn get_dual_halfword_multiply(value: Word) -> Self {
        let rd = Register::try_from((value >> 16) & 0b1111).unwrap();
        let rs = Register::try_from((value >> 8) & 0b1111).unwrap();
        let x = BitState::from(((value >> 5) & 0b1) != 0);
        let rm = Register::try_from(value & 0b1111).unwrap();

        Self::DualHalfwordMultiply { rd, rs, x, rm }
    }

    pub fn get_count_leading_zeros(value: Word) -> Self {
        let sbo1 = (value >> 16) & 0b1111;
        let rd = Register::try_from((value >> 12) & 0b1111).unwrap();
        let sbo2 = (value >> 8) & 0b1111;
        let rm = Register::try_from(value & 0b1111).unwrap();

        if sbo1 != 0b1111 {
            todo!("[SBO 1] A4.1.13 (page 175)");
        } else if sbo2 != 0b1111 {
            todo!("[SBO 2] A4.1.13 (page 175)");
        }

        Self::CountLeadingZeros { rd, rm }
    }

    pub fn get_mrs(value: Word) -> Self {
        let r = BitState::from(((value >> 22) & 0b1) != 0);
        let sbo = (value >> 16) & 0b1111;
        let rd = Register::try_from((value >> 12) & 0b1111).unwrap();
        let sbz = value & 0b1111_1111_1111;

        if sbo != 0b1111 {
            todo!("[SBO] A4.1.38 (page 224)");
        } else if sbz != 0 {
            todo!("[SBZ] A4.1.38 (page 224)");
        }

        Self::MRS { r, rd }
    }

    pub fn get_msr(value: Word) -> Self {
        let r = BitState::from(((value >> 22) & 0b1) != 0);
        let field_mask = u8::try_from((value >> 16) & 0b1111).unwrap();
        let sbo = (value >> 12) & 0b1111;
        let msr_type = {
            let should_immediate = ((value >> 25) & 0b1) != 0;

            if should_immediate {
                MSRType::get_immediate(value)
            } else {
                MSRType::get_register(value)
            }
        };

        if sbo != 0b1111 {
            todo!("[SBO] A4.1.39 (page 226)");
        }

        Self::MSR {
            r,
            field_mask,
            msr_type,
        }
    }

    pub fn get_cps(value: Word) -> Self {
        let imod = u8::try_from((value >> 18) & 0b11).unwrap();
        let mmod = BitState::from(((value >> 17) & 0b1) != 0);
        let sbz = (value >> 9) & 0b1111_111;
        let a = BitState::from(((value >> 8) & 0b1) != 0);
        let i = BitState::from(((value >> 7) & 0b1) != 0);
        let f = BitState::from(((value >> 6) & 0b1) != 0);
        let mode = OperatingMode::from(value);

        if sbz != 0 {
            todo!("[SBZ] A4.1.16 (page 179)");
        }

        Self::CPS {
            imod,
            mmod,
            a,
            i,
            f,
            mode,
        }
    }

    pub fn get_semaphore(value: Word) -> Self {
        let rn = Register::try_from((value >> 16) & 0b1111).unwrap();
        let rd = Register::try_from((value >> 12) & 0b1111).unwrap();
        let sbz = (value >> 8) & 0b1111;
        let rm = Register::try_from(value & 0b1111).unwrap();

        if sbz != 0 {
            todo!("[SBZ] A4.1.108 (page 362)");
        }

        Self::Semaphore { rn, rd, rm }
    }

    pub fn get_swi(value: Word) -> Self {
        let immed = value & ((1 << 25) - 1);
        Self::SWI(immed)
    }

    pub fn get_bkpt(value: Word) -> Self {
        let immed1 = u16::try_from((value >> 8) & 0b1111_1111_1111).unwrap();
        let immed2 = u8::try_from(value & 0b1111).unwrap();

        Self::BKPT { immed1, immed2 }
    }

    pub fn get_cpd(value: Word) -> Self {
        let opcode1 = CPOpcode::try_from((value >> 20) & 0b1111).unwrap();
        let crn = CPRegister::try_from((value >> 16) & 0b1111).unwrap();
        let crd = CPRegister::try_from((value >> 12) & 0b1111).unwrap();
        let cp_num = CPNum::try_from((value >> 8) & 0b1111).unwrap();
        let opcode2 = CPOpcode::try_from((value >> 5) & 0b111).unwrap();
        let crm = CPRegister::try_from(value & 0b1111).unwrap();

        Self::CPD {
            opcode1,
            crn,
            crd,
            cp_num,
            opcode2,
            crm,
        }
    }

    pub fn get_mcr(value: Word) -> Self {
        let opcode1 = CPOpcode::try_from((value >> 21) & 0b111).unwrap();
        let crn = CPRegister::try_from((value >> 16) & 0b1111).unwrap();
        let rd = Register::try_from((value >> 12) & 0b1111).unwrap();
        let cp_num = CPNum::try_from((value >> 8) & 0b1111).unwrap();
        let opcode2 = CPOpcode::try_from((value >> 5) & 0b111).unwrap();
        let crm = CPRegister::try_from(value & 0b1111).unwrap();

        ArmOperand::MCR {
            opcode1,
            crn,
            rd,
            cp_num,
            opcode2,
            crm,
        }
    }

    pub fn get_mcrr(value: Word) -> Self {
        let rn = Register::try_from((value >> 16) & 0b1111).unwrap();
        let rd = Register::try_from((value >> 12) & 0b1111).unwrap();
        let cp_num = CPNum::try_from((value >> 8) & 0b1111).unwrap();
        let opcode = CPOpcode::try_from((value >> 4) & 0b1111).unwrap();
        let crm = CPRegister::try_from(value & 0b1111).unwrap();

        Self::MCRR {
            rn,
            rd,
            cp_num,
            opcode,
            crm
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::cpus::general::{
        instruction::arm::{
            encoding_fields::MSRType,
            BitState,
            CPNum,
            CPOpcode,
            CPRegister,
            Register,
        },
        OperatingMode,
    };

    use super::ArmOperand;

    #[test]
    fn get_branch() {
        let word = 0b0000_0000_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmOperand::get_branch(word),
            ArmOperand::Branch(0b1111_1111_1111_1111_1111_1111)
        );
    }

    #[test]
    fn get_normal_multiply() {
        let word = 0b0000_0000_0000_1_1111_0000_1111_1001_1111;

        assert_eq!(
            ArmOperand::get_normal_multiply(word),
            ArmOperand::NormalMultiply {
                s: true,
                rd: Register::from(0b1111),
                rs: Register::from(0b1111),
                rm: Register::from(0b1111),
            }
        );
    }

    #[test]
    #[should_panic]
    fn get_normal_multiply_sbz() {
        let word = 0b0000_0000_0000_0_0000_1111_0000_0000_0000;

        ArmOperand::get_normal_multiply(word);
    }

    #[test]
    fn get_long_multiply() {
        let word = 0b0000_0000_0000_1111_1111_1111_1001_1111;

        assert_eq!(
            ArmOperand::get_long_multiply(word),
            ArmOperand::LongMultiply {
                rdhi: 0b1111,
                rdlo: 0b1111,
                rs: Register::from(0b1111),
                rm: Register::from(0b1111),
            }
        );
    }

    #[test]
    fn get_halfword_multiply() {
        let word = 0b0000_0000_0000_1111_0000_1111_1110_1111;

        assert_eq!(
            ArmOperand::get_halfword_multiply(word),
            ArmOperand::HalfwordMultiply {
                rd: Register::from(0b1111),
                rs: Register::from(0b1111),
                y: BitState::from(true),
                x: BitState::from(true),
                rm: Register::from(0b1111)
            }
        );
    }

    #[test]
    #[should_panic]
    fn get_halfword_multiply_sbz() {
        let word = 0b0000_0000_0000_0000_1111_0000_0000_0000;

        ArmOperand::get_halfword_multiply(word);
    }

    #[test]
    fn get_word_halfword_multiply() {
        let word = 0b0000_0001_0010_1111_0000_1111_1110_1111;

        assert_eq!(
            ArmOperand::get_word_halfword_multiply(word),
            ArmOperand::WordHalfwordMultiply {
                rd: Register::from(0b1111),
                rs: Register::from(0b1111),
                rm: Register::from(0b1111),
                y: BitState::from(true),
            }
        );
    }

    #[test]
    fn get_most_significant_word_multiply() {
        let word = 0b0000_0111_0101_1111_1111_1111_0011_1111;

        assert_eq!(
            ArmOperand::get_most_significant_word_multiply(word),
            ArmOperand::MostSignificantWordMultiply {
                rd: Register::from(0b1111),
                rs: Register::from(0b1111),
                rm: Register::from(0b1111),
                r: BitState::from(true),
            }
        );
    }

    #[test]
    fn get_dual_halfword_multiply() {
        let word = 0b0000_0111_0000_1111_1111_1111_0011_1111;

        assert_eq!(
            ArmOperand::get_dual_halfword_multiply(word),
            ArmOperand::DualHalfwordMultiply {
                rd: Register::from(0b1111),
                rs: Register::from(0b1111),
                x: BitState::from(true),
                rm: Register::from(0b1111),
            }
        );
    }

    #[test]
    fn get_count_leading_zeros() {
        let word = 0b0000_0001_0110_1111_1111_1111_0001_1111;

        assert_eq!(
            ArmOperand::get_count_leading_zeros(word),
            ArmOperand::CountLeadingZeros {
                rd: Register::from(0b1111),
                rm: Register::from(0b1111),
            }
        );
    }

    #[test]
    #[should_panic]
    fn get_count_leading_zeros_sbo1() {
        let word = 0b0000_0001_0110_0000_1111_1111_0001_1111;
        ArmOperand::get_count_leading_zeros(word);
    }

    #[test]
    #[should_panic]
    fn get_count_leading_zeros_sbo2() {
        let word = 0b0000_0001_0110_1111_1111_0000_0001_1111;
        ArmOperand::get_count_leading_zeros(word);
    }

    #[test]
    fn get_mrs() {
        let word = 0b0000_0001_0100_1111_1111_0000_0000_0000;

        assert_eq!(
            ArmOperand::get_mrs(word),
            ArmOperand::MRS {
                r: BitState::from(true),
                rd: Register::from(0b1111),
            }
        );
    }

    #[test]
    #[should_panic]
    fn get_mrs_sbo() {
        let word = 0b0000_0001_0100_0000_1111_0000_0000_0000;
        ArmOperand::get_mrs(word);
    }

    #[test]
    #[should_panic]
    fn get_mrs_sbz() {
        let word = 0b0000_0001_0100_1111_1111_1111_1111_1111;
        ArmOperand::get_mrs(word);
    }

    #[test]
    fn get_msr_immediate() {
        let word = 0b0000_0011_0110_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmOperand::get_msr(word),
            ArmOperand::MSR {
                r: BitState::from(true),
                field_mask: 0b1111,
                msr_type: MSRType::Immediate {
                    rotate_imm: 0b1111,
                    immediate: 0b1111_1111
                }
            }
        );
    }

    #[test]
    fn get_msr_register() {
        let word = 0b0000_0001_0110_1111_1111_00000_0000_1111;

        assert_eq!(
            ArmOperand::get_msr(word),
            ArmOperand::MSR {
                r: BitState::from(true),
                field_mask: 0b1111,
                msr_type: MSRType::Register(0b1111),
            }
        );
    }

    #[test]
    #[should_panic]
    fn get_msr_immediate_sbo() {
        let word = 0b0000_0011_0110_1111_0000_1111_1111_1111;
        ArmOperand::get_msr(word);
    }

    #[test]
    #[should_panic]
    fn get_msr_register_sbo() {
        let word = 0b0000_0001_0110_1111_0000_0000_0000_1111;
        ArmOperand::get_msr(word);
    }

    #[test]
    #[should_panic]
    fn get_msr_register_sbz() {
        let word = 0b0000_0001_0110_1111_1111_0000_0000_1111;
        ArmOperand::get_msr(word);
    }

    #[test]
    fn get_cps() {
        let word = 0b1111_0001_0000_1110_0000_0001_1100_0000;

        assert_eq!(
            ArmOperand::get_cps(word),
            ArmOperand::CPS {
                imod: 0b11,
                mmod: BitState::from(true),
                a: BitState::from(true),
                i: BitState::from(true),
                f: BitState::from(true),
                mode: OperatingMode::Usr
            }
        );
    }

    #[test]
    fn get_cps_sbz() {
        let word = 0b1111_0001_0000_1110_1111_1111_1100_0000;
        ArmOperand::get_cps(word);
    }

    #[test]
    fn get_semaphore() {
        let word = 0b1111_0001_0000_1111_1111_0000_1001_1111;

        assert_eq!(
            ArmOperand::get_semaphore(word),
            ArmOperand::Semaphore {
                rn: Register::from(0b1111),
                rd: Register::from(0b1111),
                rm: Register::from(0b11111),
            }
        );
    }

    #[test]
    #[should_panic]
    fn get_semphore_sbz() {
        let word = 0b0000_0001_0000_1111_1111_1111_1001_1111;
        ArmOperand::get_semaphore(word);
    }

    #[test]
    fn get_swi() {
        let word = 0b0000_1111_1111_0000_1111_0000_1111_0000;

        assert_eq!(
            ArmOperand::get_swi(word),
            ArmOperand::SWI(0b1111_0000_1111_0000_1111_0000)
        );
    }

    #[test]
    fn get_bkpt() {
        let word = 0b1110_0001_0010_1111_1111_1111_0111_1111;

        assert_eq!(
            ArmOperand::get_bkpt(word),
            ArmOperand::BKPT {
                immed1: 0b1111_1111_1111,
                immed2: 0b1111,
            }
        );
    }

    #[test]
    fn get_cpd() {
        let word = 0b0000_1110_1111_1111_1111_1111_1110_1111;

        assert_eq!(
            ArmOperand::get_cpd(word),
            ArmOperand::CPD {
                opcode1: CPOpcode::from(0b1111),
                crn: CPRegister::from(0b1111),
                crd: CPRegister::from(0b1111),
                cp_num: CPNum::from(0b1111),
                opcode2: CPOpcode::from(0b111),
                crm: CPRegister::from(0b1111),
            }
        );
    }

    #[test]
    fn get_mcr() {
        let word = 0b0000_1110_1110_1111_1111_1111_1111_1111;

        assert_eq!(
        ArmOperand::get_mcr(word),
        ArmOperand::MCR {
            opcode1: CPOpcode::from(0b111),
            crn: CPRegister::from(0b1111),
            rd: Register::from(0b1111),
            cp_num: CPNum::from(0b1111),
            opcode2: CPOpcode::from(0b111),
            crm: CPRegister::from(0b1111),
        }
        );
    }

    #[test]
    fn get_mcrr() {
        let word = 0b0000_1100_0100_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmOperand::get_mcrr(word),
            ArmOperand::MCRR {
                rn: Register::from(0b1111),
                rd: Register::from(0b1111),
                cp_num: CPNum::from(0b1111),
                opcode: CPOpcode::from(0b1111),
                crm: CPOpcode::from(0b1111),
            }
        );
    }
}
