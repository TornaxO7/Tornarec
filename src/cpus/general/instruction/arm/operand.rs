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
        mmod: u8,
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
        rn: BitState,
        rd: BitState,
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

        Self::CountLeadingZeros {
            rd,
            rm,
        }
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

        Self::MRS {
            r,
            rd
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::cpus::general::instruction::arm::{
        BitState,
        Register,
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
}
