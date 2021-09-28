use crate::cpus:: {
    state::State,
    arm7tdmi:: {
        interruption::Interruption,
        operating_mode::OperatingMode,
        operating_state::OperatingState,
    },
};

use core::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionCodeFlag {EQ, NE, CS, CC, MI, PL, VS, VC, HI, LS, GE, LT, GT, LE, AL}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum CpsrError {
    #[error("The following operating mode is unknown: {0:b}")]
    UnknownOperatingMode(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cpsr(u32);

impl Cpsr {

    pub fn get_condition(&self, condition: ConditionCodeFlag) -> bool {

        let n = State::from(self.0 >> 31);
        let z = State::from((self.0 >> 30) & 1);
        let c = State::from((self.0 >> 29) & 1);
        let v = State::from((self.0 >> 28) & 1);

        match condition {
            ConditionCodeFlag::EQ => z.is_set(),
            ConditionCodeFlag::NE => z.is_unset(),
            ConditionCodeFlag::CS => c.is_set(),
            ConditionCodeFlag::CC => c.is_unset(),
            ConditionCodeFlag::MI => n.is_set(),
            ConditionCodeFlag::PL => n.is_unset(),
            ConditionCodeFlag::VS => v.is_set(),
            ConditionCodeFlag::VC => v.is_unset(),
            ConditionCodeFlag::HI => c.is_set() && z.is_unset(),
            ConditionCodeFlag::LS => c.is_unset() && z.is_set(),
            ConditionCodeFlag::GE => n == v,
            ConditionCodeFlag::LT => n != v,
            ConditionCodeFlag::GT => z.is_unset() && n == v,
            ConditionCodeFlag::LE => z.is_set() || n != v,
            ConditionCodeFlag::AL => true,
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

    pub fn set_operating_state(&mut self, operating_state: OperatingState) {
        match operating_state {
            OperatingState::Arm => self.0 &= !(1 << 5),
            OperatingState::Thumb => self.0 |= 1 << 5,
        };
    }

    pub fn get_operating_state(&self) -> OperatingState {
        if (self.0 >> 5) & 1 == 0 {
            OperatingState::Arm
        } else {
            OperatingState::Thumb
        }
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

impl From<u32> for Cpsr {
    fn from(num: u32) -> Self {
        Self(num)
    }
}

#[cfg(test)]
mod tests {

    use crate::cpus::arm7tdmi::Interruption;
    use crate::cpus::state::State;
    use super::{Cpsr, ConditionCodeFlag, OperatingMode, CpsrError, OperatingState};

    #[test]
    fn get_condition() {
        let cpsr = Cpsr::from(0b1011_0000_0000_0000_0000_0000_0000_0000);

        assert!(!cpsr.get_condition(ConditionCodeFlag::EQ));
        assert!(cpsr.get_condition(ConditionCodeFlag::NE));
        assert!(cpsr.get_condition(ConditionCodeFlag::CS));
        assert!(!cpsr.get_condition(ConditionCodeFlag::CC));
        assert!(cpsr.get_condition(ConditionCodeFlag::MI));
        assert!(!cpsr.get_condition(ConditionCodeFlag::PL));
        assert!(cpsr.get_condition(ConditionCodeFlag::VS));
        assert!(!cpsr.get_condition(ConditionCodeFlag::VC));
        assert!(cpsr.get_condition(ConditionCodeFlag::HI));
        assert!(!cpsr.get_condition(ConditionCodeFlag::LS));
        assert!(cpsr.get_condition(ConditionCodeFlag::GE));
        assert!(!cpsr.get_condition(ConditionCodeFlag::LT));
        assert!(cpsr.get_condition(ConditionCodeFlag::GT));
        assert!(!cpsr.get_condition(ConditionCodeFlag::LE));
        assert!(cpsr.get_condition(ConditionCodeFlag::AL));
    }

    #[test]
    fn set_interrupt_bit() {
        let mut cpsr_irq_set = Cpsr::from(0);
        let mut cpsr_irq_unset = Cpsr::from(0);
        let mut cpsr_fiq_set = Cpsr::from(0);
        let mut cpsr_fiq_unset = Cpsr::from(0);

        cpsr_irq_set.set_interrupt_bit(Interruption::Irq, State::Set);
        cpsr_irq_unset.set_interrupt_bit(Interruption::Irq, State::Unset);

        cpsr_fiq_set.set_interrupt_bit(Interruption::Fiq, State::Set);
        cpsr_fiq_unset.set_interrupt_bit(Interruption::Fiq, State::Unset);

        assert_eq!(cpsr_irq_set, Cpsr::from(0b1000_0000));
        assert_eq!(cpsr_irq_unset, Cpsr::from(0b0000_0000));
        assert_eq!(cpsr_fiq_set, Cpsr::from(0b100_0000));
        assert_eq!(cpsr_fiq_unset, Cpsr::from(0b000_0000));
    }

    #[test]
    fn get_interrupt_bit() {
        let cpsr_irq_set = Cpsr::from(0b1000_0000);
        let cpsr_fiq_set = Cpsr::from(0b100_0000);
        let cpsr_irq_unset = Cpsr::from(0b1111_1111_1111_1111_1111_1111_0111_1111);
        let cpsr_fiq_unset = Cpsr::from(0b1111_1111_1111_1111_1111_1111_1011_1111);

        assert_eq!(cpsr_irq_set.get_interrrupt_bit_state(Interruption::Irq), State::Set);
        assert_eq!(cpsr_fiq_set.get_interrrupt_bit_state(Interruption::Fiq), State::Set);

        assert_eq!(cpsr_irq_unset.get_interrrupt_bit_state(Interruption::Irq), State::Unset);
        assert_eq!(cpsr_fiq_unset.get_interrrupt_bit_state(Interruption::Fiq), State::Unset);
    }

    #[test]
    fn set_operating_state() {
        let mut cpsr_arm = Cpsr::from(0);
        let mut cpsr_thumb = Cpsr::from(0);

        cpsr_arm.set_operating_state(OperatingState::Arm);
        cpsr_thumb.set_operating_state(OperatingState::Thumb);

        assert_eq!(cpsr_arm, Cpsr::from(0b000000));
        assert_eq!(cpsr_thumb, Cpsr::from(0b100000));
    }

    #[test]
    fn get_operating_state() {
        let cpsr_arm = Cpsr::from(0b1111_1111_1111_1111_1111_1111_1101_1111);
        let cpsr_thumb = Cpsr::from(0b0000_0000_0000_0000_0000_0000_0010_0000);

        assert_eq!(cpsr_arm.get_operating_state(), OperatingState::Arm);
        assert_eq!(cpsr_thumb.get_operating_state(), OperatingState::Thumb);
    }


    #[test]
    fn set_operating_mode() {
        // it shouldn't matter which previous it had, so we're setting the mode with some random
        // bits
        let mut cpsr_usr = Cpsr::from(0);
        let mut cpsr_fiq = Cpsr::from(0b11111);
        let mut cpsr_irq = Cpsr::from(0b10101);
        let mut cpsr_svc = Cpsr::from(0b11010);
        let mut cpsr_abt = Cpsr::from(0b10100);
        let mut cpsr_sys = Cpsr::from(0b10111);
        let mut cpsr_und = Cpsr::from(0b10010);

        cpsr_usr.set_operating_mode(OperatingMode::Usr);
        cpsr_fiq.set_operating_mode(OperatingMode::Fiq);
        cpsr_irq.set_operating_mode(OperatingMode::Irq);
        cpsr_svc.set_operating_mode(OperatingMode::Svc);
        cpsr_abt.set_operating_mode(OperatingMode::Abt);
        cpsr_sys.set_operating_mode(OperatingMode::Sys);
        cpsr_und.set_operating_mode(OperatingMode::Und);

        assert_eq!(cpsr_usr, Cpsr::from(0b10000));
        assert_eq!(cpsr_fiq, Cpsr::from(0b10001));
        assert_eq!(cpsr_irq, Cpsr::from(0b10010));
        assert_eq!(cpsr_svc, Cpsr::from(0b10011));
        assert_eq!(cpsr_abt, Cpsr::from(0b10111));
        assert_eq!(cpsr_sys, Cpsr::from(0b11011));
        assert_eq!(cpsr_und, Cpsr::from(0b11111));
    }

    #[test]
    fn get_operating_mode() {
        let cpsr_usr = Cpsr::from(0b10000);
        let cpsr_fiq = Cpsr::from(0b10001);
        let cpsr_irq = Cpsr::from(0b10010);
        let cpsr_svc = Cpsr::from(0b10011);
        let cpsr_abt = Cpsr::from(0b10111);
        let cpsr_und = Cpsr::from(0b11011);
        let cpsr_sys = Cpsr::from(0b11111);
        let cpsr_unknown = Cpsr::from(0b10101);

        assert_eq!(cpsr_usr.get_operating_mode(), Ok(OperatingMode::Usr));
        assert_eq!(cpsr_fiq.get_operating_mode(), Ok(OperatingMode::Fiq));
        assert_eq!(cpsr_irq.get_operating_mode(), Ok(OperatingMode::Irq));
        assert_eq!(cpsr_svc.get_operating_mode(), Ok(OperatingMode::Svc));
        assert_eq!(cpsr_abt.get_operating_mode(), Ok(OperatingMode::Abt));
        assert_eq!(cpsr_und.get_operating_mode(), Ok(OperatingMode::Und));
        assert_eq!(cpsr_sys.get_operating_mode(), Ok(OperatingMode::Sys));
        assert_eq!(cpsr_unknown.get_operating_mode(), Err(CpsrError::UnknownOperatingMode(0b10101)));
    }

}
