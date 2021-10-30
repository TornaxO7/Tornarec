#[derive(Debug, Clone, Copy, Eq)]
#[rustfmt::skip]
pub enum RegisterName {
    // general purpose registers
    R0, R1, R2, R3, R4, R5, R6, R7,

    // banked fiq registers
    R8, R8Fiq,
    R9, R9Fiq,
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
    SpsrSvc,
    SpsrAbt,
    SpsrUnd,
    SpsrIrq,
    SpsrFiq,
}

impl RegisterName {
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

    fn normalize(reg: &RegisterName) -> RegisterName {
        match reg {
            RegisterName::Sp => RegisterName::R13,
            RegisterName::SpSvc => RegisterName::R13Svc,
            RegisterName::SpAbt => RegisterName::R13Abt,
            RegisterName::SpUnd => RegisterName::R13Und,
            RegisterName::SpIrq => RegisterName::R13Irq,
            RegisterName::SpFiq => RegisterName::R13Fiq,

            RegisterName::Lr => RegisterName::R14,
            RegisterName::LrSvc => RegisterName::R14Svc,
            RegisterName::LrAbt => RegisterName::R14Abt,
            RegisterName::LrUnd => RegisterName::R14Und,
            RegisterName::LrIrq => RegisterName::R14Irq,
            RegisterName::LrFiq => RegisterName::R14Fiq,

            RegisterName::Pc => RegisterName::R15,
            other => *other,
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
            other => unreachable!(
                "[Register Name Error]: '{:b}' is an unknown register.",
                other
            ),
        }
    }
}

impl From<u8> for RegisterName {
    fn from(num: u8) -> Self {
        Self::from(u32::from(num))
    }
}

impl PartialEq for RegisterName {
    fn eq(&self, other: &Self) -> bool {
        let self_normalized = Self::normalize(self);
        let other_normalized = Self::normalize(other);

        self_normalized as u32 == other_normalized as u32
    }
}

#[cfg(test)]
mod tests {

    use super::RegisterName;

    #[test]
    fn compare() {
        assert!(RegisterName::R13 == RegisterName::Sp);
        assert!(RegisterName::R13Svc == RegisterName::SpSvc);
        assert!(RegisterName::R13Abt == RegisterName::SpAbt);
        assert!(RegisterName::R13Und == RegisterName::SpUnd);
        assert!(RegisterName::R13Irq == RegisterName::SpIrq);
        assert!(RegisterName::R13Fiq == RegisterName::SpFiq);

        assert!(RegisterName::R14 == RegisterName::Lr);
        assert!(RegisterName::R14Svc == RegisterName::LrSvc);
        assert!(RegisterName::R14Abt == RegisterName::LrAbt);
        assert!(RegisterName::R14Und == RegisterName::LrUnd);
        assert!(RegisterName::R14Irq == RegisterName::LrIrq);
        assert!(RegisterName::R14Fiq == RegisterName::LrFiq);

        assert!(RegisterName::R15 == RegisterName::Pc);
    }
}
