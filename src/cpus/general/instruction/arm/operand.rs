use crate::cpus::general::OperatingMode;

use super::{
    encoding_fields::{
        AddressingMode1Offset,
        AddressingMode2Offset,
        AddressingMode3Offset,
        AddressingMode4Offset,
        AddressingMode5Offset,
        RegisterList, MSRType,
    },
    BitState,
    CPNum,
    CPRegister,
    Register,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmOperand {
    Branch(u32),
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
        offset: u8,
        offset_type: AddressingMode5Offset,
    },
}
