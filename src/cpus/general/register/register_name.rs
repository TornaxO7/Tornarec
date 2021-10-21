#[derive(Debug, Clone, Eq)]
pub enum RegisterName {
    // general purpose registers
    R0, R1, R2, R3, R4, R5, R6, R7,

    // banked fiq registers
    R8,  R8Fiq,
    R9,  R9Fiq,
    R10, R10Fiq,
    R11, R11Fiq,
    R12, R12Fiq,

    // Format: <Register Index>, <Aliasname>
    R13, Sp,
    R13Svc, SpSvc,
    R13Abt, SpAbt,
    R13Und, SpUnd,
    R13Irq, SpIrq,
    R13Fiq, SpFiq,

    // Format: <Register Index>, <Aliasname>
    R14, Lr,
    R14Svc, LrSvc,
    R14Abt, LrAbt,
    R14Und, LrUnd,
    R14Irq, LrIrq,
    R14Fiq, LrFiq,

    // Format: <Register Index>, <Aliasname>
    R15, Pc,

    Cpsr,
    SpsrSvc, SpsrAbt, SpsrUnd, SpsrIrq, SpsrFiq
}

impl RegisterName {

    fn normalize(name: &Self) -> Self {
        match name {
            Self::Sp => Self::R13,
            Self::SpSvc => Self::R13Svc,
            Self::SpAbt => Self::R13Abt,
            Self::SpUnd => Self::R13Und,
            Self::SpIrq => Self::R13Irq,
            Self::SpFiq => Self::R13Fiq,

            Self::Lr => Self::R14,
            Self::LrSvc => Self::R14Svc,
            Self::LrAbt => Self::R14Abt,
            Self::LrUnd => Self::R14Und,
            Self::LrIrq => Self::R14Irq,
            Self::LrFiq => Self::R14Fiq,

            Self::Pc => Self::R15,
            _other => _other.clone(),
        }
    }

    pub fn get_as_u32(&self) -> u32 {
        match self {
            Self::R0 => 0b0000,
            Self::R1 => 0b0001,
            Self::R2 => 0b0010,
            Self::R3 => 0b0011,
            Self::R4 => 0b0100,
            Self::R5 => 0b0101,
            Self::R6 => 0b0110,
            Self::R7 => 0b0111,
            Self::R8 | Self::R8Fiq => 0b1000,
            Self::R9 | Self::R9Fiq => 0b1001,
            Self::R10 | Self::R10Fiq => 0b1010,
            Self::R11 | Self::R11Fiq => 0b1011,
            Self::R12 | Self::R12Fiq => 0b1100,

            Self::R13 | Self::Sp 
                | Self::R13Svc | Self::SpSvc
                | Self::R13Abt | Self::SpAbt
                | Self::R13Und | Self::SpUnd
                | Self::R13Irq | Self::SpIrq
                | Self::R13Fiq | Self::SpFiq => 0b1101,

            Self::R14 | Self::Lr 
                | Self::R14Svc | Self::LrSvc
                | Self::R14Abt | Self::LrAbt
                | Self::R14Und | Self::LrUnd
                | Self::R14Irq | Self::LrIrq
                | Self::R14Fiq | Self::LrFiq => 0b1110,

            Self::R15 | Self::Pc => 0b1111,

            _other => unreachable!("[Register Name error]: CPSR and co shouldn't be have a binary representation. Register was: '{:?}'.", _other),
        }
    }
}

impl From<u32> for RegisterName {
    fn from(num: u32) -> Self {
        match num {
            0b0000 => Self::R0, 
            0b0001 => Self::R1, 
            0b0010 => Self::R2, 
            0b0011 => Self::R3, 
            0b0100 => Self::R4, 
            0b0101 => Self::R5, 
            0b0110 => Self::R6, 
            0b0111 => Self::R7, 
            0b1000 => Self::R8, 
            0b1001 => Self::R9, 
            0b1010 => Self::R10, 
            0b1011 => Self::R11, 
            0b1100 => Self::R12, 
            0b1101 => Self::R13, 
            0b1110 => Self::R14, 
            0b1111 => Self::R15, 
            _other => unreachable!("[Register Name Error]: '{:b}' is an unknown register.", _other),
        }
    }
}

impl PartialEq for RegisterName {
    fn eq(&self, other: &Self) -> bool {
        let current = Self::normalize(self);
        let other = Self::normalize(other);

        current == other
    }
}
