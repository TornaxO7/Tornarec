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
}

impl OperatingMode {
    pub const AMOUNT_MODES: usize = 7;

    pub const fn as_u32(operating_mode: OperatingMode) -> u32 {
        match operating_mode {
            OperatingMode::Usr | OperatingMode::Sys => 0,
            OperatingMode::Fiq => 1,
            OperatingMode::Irq => 2,
            OperatingMode::Svc => 3,
            OperatingMode::Abt => 4,
            OperatingMode::Und => 5,
        }
    }

    pub const fn as_usize(operating_mode: OperatingMode) -> usize {
        match operating_mode {
            OperatingMode::Usr | OperatingMode::Sys => 0,
            OperatingMode::Fiq => 1,
            OperatingMode::Irq => 2,
            OperatingMode::Svc => 3,
            OperatingMode::Abt => 4,
            OperatingMode::Und => 5,
        }
    }
}
