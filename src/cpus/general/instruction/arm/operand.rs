use super::{
    encoding_fields::{
        AddressingMode1Offset,
        AddressingMode2Offset,
        AddressingMode3Offset,
        AddressingMode4Offset,
        AddressingMode5Offset,
        RegisterList,
    },
    BitState,
    CPNum,
    CPRegister,
    Register,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmOperand {
    Branch(u32),
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
