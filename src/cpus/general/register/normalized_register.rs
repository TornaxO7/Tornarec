use super::RegisterName;

use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedRegister(RegisterName);

impl From<RegisterName> for NormalizedRegister {
    fn from(register_name: RegisterName) -> Self {

        let register_name = match register_name {
            RegisterName::Sp => RegisterName::R13,
            RegisterName::SpSvc => RegisterName::R13Svc,
            RegisterName::SpAbt => RegisterName::R13Abt,
            RegisterName::SpUnd => RegisterName::R13Und,
            RegisterName::SpIrq => RegisterName::R13Irq,
            RegisterName::SpFiq => RegisterName::R13Fiq,

            RegisterName::Lr => RegisterName::R14,
            RegisterName::LrSvc => RegisterName::R14Svc,
            RegisterName::LrAbt => RegisterName::R14Abt,
            RegisterName::LrUnd => RegisterName::R14Und,
            RegisterName::LrIrq => RegisterName::R14Irq,
            RegisterName::LrFiq => RegisterName::R14Fiq,

            RegisterName::Pc => RegisterName::R15,
            _other => _other.clone(),
        };

        Self(register_name)
    }
}
