#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FiqReg {reg: u32, fiq: u32}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Spsr {
    fiq: u32,
    svc: u32,
    abt: u32,
    irq: u32,
    und: u32,
}

