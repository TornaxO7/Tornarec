#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum OperatingMode {
    Usr,
    Fiq,
    Irq,
    Svc,
    Abt,
    Sys,
    Und,
    AmountModes
}
