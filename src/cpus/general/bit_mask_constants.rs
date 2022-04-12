use crate::cpus::Architecture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitMaskConstants {
    UnallocMask,
    UserMask,
    PrivMask,
    StateMask,
}

impl BitMaskConstants {
    pub fn as_u32(&self, architecture: Architecture) -> u32 {
        match self {
            Self::UnallocMask => match architecture {
                Architecture::ARMv4T => 0x0FFF_FF20,
                Architecture::ARMv5TE => 0x07FF_FF00,
            },
            Self::UserMask => match architecture {
                Architecture::ARMv4T => 0xF000_0000,
                Architecture::ARMv5TE => 0xF800_0000,
            },
            Self::PrivMask => match architecture {
                Architecture::ARMv4T => 0x0000_000F,
                Architecture::ARMv5TE => 0x0000_000F,
            },
            Self::StateMask => match architecture {
                Architecture::ARMv4T => 0x0,
                Architecture::ARMv5TE => 0x0000_0020,
            },
        }
    }
}
