use crate::cpus::general::{
    register::Cpsr,
    operating_mode::OperatingMode,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Spsr {
    pub fiq: Cpsr,
    pub svc: Cpsr,
    pub abt: Cpsr,
    pub irq: Cpsr,
    pub und: Cpsr,
}

impl Spsr {
    pub fn set_value(&mut self, value: Cpsr, operating_mode: OperatingMode) {
        match operating_mode {
            OperatingMode::Fiq => self.fiq = value,
            OperatingMode::Svc => self.svc = value,
            OperatingMode::Abt => self.abt = value,
            OperatingMode::Irq => self.irq = value,
            OperatingMode::Und => self.und = value,
            _other => unreachable!("[SPSR ERROR]: Operating mode '{}' doesn't have an SPSR.", _other),
        }
    }
}
