use crate::cpus:: {
    state::State,
    arm7tdmi:: {
        interruption::Interruption,
        operating_mode::OperatingMode,
        operating_state::OperatingState,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {EQ, NE, CS, CC, MI, PL, VS, VC, HI, LS, GE, LT, GT, LE, AL}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum CpsrError {
    #[error("The following operating mode is unknown: {0:b}")]
    UnknownOperatingMode(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cpsr(u32);

impl Cpsr {

    pub fn get_condition(&self, condition: Condition) -> bool {

        let n = State::from(self.0 >> 31);
        let z = State::from((self.0 >> 30) & 1);
        let c = State::from((self.0 >> 29) & 1);
        let v = State::from((self.0 >> 28) & 1);

        let less_than = (n.is_set() && v.is_unset()) || (n.is_unset() && v.is_set());

        match condition {
            Condition::EQ => z.is_set(),
            Condition::NE => z.is_unset(),
            Condition::CS => c.is_set(),
            Condition::CC => c.is_unset(),
            Condition::MI => n.is_set(),
            Condition::PL => n.is_unset(),
            Condition::VS => v.is_set(),
            Condition::VC => v.is_unset(),
            Condition::HI => c.is_set() && z.is_unset(),
            Condition::LS => c.is_unset() && z.is_set(),
            Condition::GE => n == v,
            Condition::LT => less_than,
            Condition::GT => z.is_unset() && n == v,
            Condition::LE => z.is_set() || less_than,
            Condition::AL => true,
        }
    }

    pub fn set_interrupt_bit(&mut self, interrupt: Interruption, state: State) {
        match interrupt {
            Interruption::Irq => match state {
                State::Unset => self.0 &= !(1 << 7),
                State::Set   => self.0 |= 1 << 7,
            },
            Interruption::Fiq => match state {
                State::Unset => self.0 &= !(1 << 6),
                State::Set   => self.0 |= 1 << 6,
            }
        }
    }

    pub fn get_interrrupt_bit_state(&self, interrupt: Interruption) -> State {
        match interrupt {
            Interruption::Irq => {
                if (self.0 >> 7) & 1 == 1 {
                    State::Set
                } else {
                    State::Unset
                }
            },
            Interruption::Fiq => {
                if (self.0 >> 6) & 1 == 1 {
                    State::Set
                } else {
                    State::Unset
                }
            },
        }
    }

    pub fn get_operating_state(&self) -> OperatingState {
        if (self.0 >> 5) & 1 == 0 {
            OperatingState::Arm
        } else {
            OperatingState::Thumb
        }
    }

    pub fn set_operating_state(&mut self, operating_state: OperatingState) {
        match operating_state {
            OperatingState::Arm => self.0 | (1 << 5),
            OperatingState::Thumb => self.0 & !(1 << 5),
        };
    }

    pub fn set_operating_mode(&mut self, operating_mode: OperatingMode) {
        self.0 &= !0b11111;
        match operating_mode {
            OperatingMode::Usr => self.0 |= 0b10000,
            OperatingMode::Fiq => self.0 |= 0b10001,
            OperatingMode::Irq => self.0 |= 0b10010,
            OperatingMode::Svc => self.0 |= 0b10011,
            OperatingMode::Abt => self.0 |= 0b10111,
            OperatingMode::Sys => self.0 |= 0b11011,
            OperatingMode::Und => self.0 |= 0b11111,
        };
    }
    
    pub fn get_operating_mode(&self) -> Result<OperatingMode, CpsrError> {
        match self.0 & 0b11111 {
            0b10000 => Ok(OperatingMode::Usr),
            0b10001 => Ok(OperatingMode::Fiq),
            0b10010 => Ok(OperatingMode::Irq),
            0b10011 => Ok(OperatingMode::Svc),
            0b10111 => Ok(OperatingMode::Abt),
            0b11011 => Ok(OperatingMode::Und),
            0b11111 => Ok(OperatingMode::Sys),
            _other => Err(CpsrError::UnknownOperatingMode(_other)),
        }
    }
}
