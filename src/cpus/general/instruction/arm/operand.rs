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
        rm: Register,
        y: BitState,
        x: BitState,
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
            todo!("SBZ, see A4.1.40");
        }

        Self::NormalMultiply { s, rd, rs, rm }
    }
}

#[cfg(test)]
mod tests {

    use crate::cpus::general::instruction::arm::Register;

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
}
