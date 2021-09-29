use crate::cpus::general::register::GeneralRegister;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FiqReg {
    reg: GeneralRegister,
    fiq: GeneralRegister,
}
