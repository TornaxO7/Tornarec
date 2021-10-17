#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionVector;

impl ExceptionVector {
    pub const SWI: u32 = 0x00000008;
    pub const UDEF: u32 = 0x00000004;
    pub const PABT: u32 = 0x0000000C;
    pub const FIQ: u32 = 0x0000001C;
    pub const IRQ: u32 = 0x00000018;
    pub const DABT: u32 = 0x00000010;
    pub const RESET: u32 = 0x00000000;
}
