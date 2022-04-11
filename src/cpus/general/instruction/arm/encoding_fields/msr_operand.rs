use crate::cpus::general::instruction::arm::Register;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MSRType {
    Immediate {
        rotate_imm: u8,
        immediate: u8,
    },
    Register(Register),
}
