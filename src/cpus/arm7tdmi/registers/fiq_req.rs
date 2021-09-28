use crate::cpus::arm7tdmi::registers::GeneralRegister;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FiqReg {
    reg: GeneralRegister,
    fiq: GeneralRegister,
}
