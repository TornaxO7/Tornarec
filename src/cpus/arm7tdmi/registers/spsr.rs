use crate::cpus::arm7tdmi::registers::Cpsr;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Spsr {
    pub fiq: Cpsr,
    pub svc: Cpsr,
    pub abt: Cpsr,
    pub irq: Cpsr,
    pub und: Cpsr,
}

