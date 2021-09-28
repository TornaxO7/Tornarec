#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exception {
    Bl,
    Swi,
    Udef,
    Pabt,
    Fiq,
    Irq,
    Dabt,
    Reset,
}
