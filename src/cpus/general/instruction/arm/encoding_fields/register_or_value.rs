use crate::cpus::general::register::RegisterName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterOrValue {
    Register(RegisterName),
    Value(u32),
}
