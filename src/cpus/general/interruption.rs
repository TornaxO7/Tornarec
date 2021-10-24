#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interruption {
    Irq,
    Fiq,
}
