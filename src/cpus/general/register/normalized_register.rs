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

#[cfg(test)]
mod tests {

    use super::{RegisterName, NormalizedRegister};

    #[test]
    fn eq() {
        assert_eq!(NormalizedRegister::from(RegisterName::Sp), NormalizedRegister::from(RegisterName::R13));
        assert_eq!(NormalizedRegister::from(RegisterName::SpSvc), NormalizedRegister::from(RegisterName::R13Svc));
        assert_eq!(NormalizedRegister::from(RegisterName::SpAbt), NormalizedRegister::from(RegisterName::R13Abt));
        assert_eq!(NormalizedRegister::from(RegisterName::SpUnd), NormalizedRegister::from(RegisterName::R13Und));
        assert_eq!(NormalizedRegister::from(RegisterName::SpIrq), NormalizedRegister::from(RegisterName::R13Irq));
        assert_eq!(NormalizedRegister::from(RegisterName::SpFiq), NormalizedRegister::from(RegisterName::R13Fiq));

        assert_eq!(NormalizedRegister::from(RegisterName::Lr), NormalizedRegister::from(RegisterName::R14));
        assert_eq!(NormalizedRegister::from(RegisterName::LrSvc), NormalizedRegister::from(RegisterName::R14Svc));
        assert_eq!(NormalizedRegister::from(RegisterName::LrAbt), NormalizedRegister::from(RegisterName::R14Abt));
        assert_eq!(NormalizedRegister::from(RegisterName::LrUnd), NormalizedRegister::from(RegisterName::R14Und));
        assert_eq!(NormalizedRegister::from(RegisterName::LrIrq), NormalizedRegister::from(RegisterName::R14Irq));
        assert_eq!(NormalizedRegister::from(RegisterName::LrFiq), NormalizedRegister::from(RegisterName::R14Fiq));
    }
}
