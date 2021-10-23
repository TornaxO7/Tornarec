use crate::cpus::general::register::NormalizedRegister;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterOrValue {
    Register(NormalizedRegister),
    Value(u32),
}
