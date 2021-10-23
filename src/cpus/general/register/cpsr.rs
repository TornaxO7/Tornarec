use crate::cpus::general:: {
    bit_state::BitState,
    operating_mode::OperatingMode,
    operating_state::OperatingState,
    interruption::Interruption,
    condition_code_flag::ConditionCodeFlag,
    register::types::{ConditionBits, ConditionBit},
};

use core::convert::From;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum CpsrError {
    #[error("The following operating mode is unknown: {0:b}")]
    UnknownOperatingMode(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cpsr(u32);

impl Cpsr {

    pub fn current_mode_has_spsr(&self) -> bool {
        ![OperatingMode::Usr, OperatingMode::Sys]
            .contains(&self.get_operating_mode().unwrap())
    }

    pub fn is_condition_set(&self, condition: ConditionCodeFlag) -> bool {

        let cb = self.get_condition_bits();

        match condition {
            ConditionCodeFlag::EQ => cb.z.is_set(),
            ConditionCodeFlag::NE => cb.z.is_unset(),
            ConditionCodeFlag::CS => cb.c.is_set(),
            ConditionCodeFlag::CC => cb.c.is_unset(),
            ConditionCodeFlag::MI => cb.n.is_set(),
            ConditionCodeFlag::PL => cb.n.is_unset(),
            ConditionCodeFlag::VS => cb.v.is_set(),
            ConditionCodeFlag::VC => cb.v.is_unset(),
            ConditionCodeFlag::HI => cb.c.is_set() && cb.z.is_unset(),
            ConditionCodeFlag::LS => cb.c.is_unset() && cb.z.is_set(),
            ConditionCodeFlag::GE => cb.n == cb.v,
            ConditionCodeFlag::LT => cb.n != cb.v,
            ConditionCodeFlag::GT => cb.z.is_unset() && cb.n == cb.v,
            ConditionCodeFlag::LE => cb.z.is_set() || cb.n != cb.v,
            ConditionCodeFlag::AL => true,
        }
    }

    pub fn get_condition_bits(&self) -> ConditionBits {
        ConditionBits::from(self.0)
    }

    pub fn set_interrupt_bit(&mut self, interrupt: Interruption, state: BitState) {
        match interrupt {
            Interruption::Irq => match state {
                BitState::Unset => self.0 &= !(1 << 7),
                BitState::Set   => self.0 |= 1 << 7,
            },
            Interruption::Fiq => match state {
                BitState::Unset => self.0 &= !(1 << 6),
                BitState::Set   => self.0 |= 1 << 6,
            }
        }
    }

    pub fn get_interrrupt_bit_state(&self, interrupt: Interruption) -> BitState {
        match interrupt {
            Interruption::Irq => {
                if (self.0 >> 7) & 1 == 1 {
                    BitState::Set
                } else {
                    BitState::Unset
                }
            },
            Interruption::Fiq => {
                if (self.0 >> 6) & 1 == 1 {
                    BitState::Set
                } else {
                    BitState::Unset
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

    pub fn set_condition_bit(&mut self, condition_bit: ConditionBit, state: BitState) {
        match condition_bit {
            ConditionBit::N => self.0 = (self.0 & !(1 << 31)) | (state.get_as_u32() << 31),
            ConditionBit::Z => self.0 = (self.0 & !(1 << 30)) | (state.get_as_u32() << 30),
            ConditionBit::C => self.0 = (self.0 & !(1 << 29)) | (state.get_as_u32() << 29),
            ConditionBit::V => self.0 = (self.0 & !(1 << 28)) | (state.get_as_u32() << 28),
        }
    }

    pub fn get_condition_bit(&self, condition_bit: ConditionBit) -> BitState {
        match condition_bit {
            ConditionBit::N => BitState::from(self.0 >> 31),
            ConditionBit::Z => BitState::from(self.0 >> 30),
            ConditionBit::C => BitState::from(self.0 >> 29),
            ConditionBit::V => BitState::from(self.0 >> 28),
        }
    }

    pub fn get_as_u32(&self) -> u32 {
        self.0
    }

    pub fn set(&mut self, new_val: u32) {
        self.0 = new_val;
    }
}

impl From<u32> for Cpsr {
    fn from(num: u32) -> Self {
        Self(num)
    }
}

#[cfg(test)]
mod tests {

    use super::{Cpsr, ConditionCodeFlag, OperatingMode, CpsrError, OperatingState, ConditionBit, BitState, Interruption};

    #[test]
    fn get_condition() {
        let cpsr = Cpsr::from(0b1011_0000_0000_0000_0000_0000_0000_0000);

        assert!(!cpsr.is_condition_set(ConditionCodeFlag::EQ));
        assert!(cpsr.is_condition_set(ConditionCodeFlag::NE));
        assert!(cpsr.is_condition_set(ConditionCodeFlag::CS));
        assert!(!cpsr.is_condition_set(ConditionCodeFlag::CC));
        assert!(cpsr.is_condition_set(ConditionCodeFlag::MI));
        assert!(!cpsr.is_condition_set(ConditionCodeFlag::PL));
        assert!(cpsr.is_condition_set(ConditionCodeFlag::VS));
        assert!(!cpsr.is_condition_set(ConditionCodeFlag::VC));
        assert!(cpsr.is_condition_set(ConditionCodeFlag::HI));
        assert!(!cpsr.is_condition_set(ConditionCodeFlag::LS));
        assert!(cpsr.is_condition_set(ConditionCodeFlag::GE));
        assert!(!cpsr.is_condition_set(ConditionCodeFlag::LT));
        assert!(cpsr.is_condition_set(ConditionCodeFlag::GT));
        assert!(!cpsr.is_condition_set(ConditionCodeFlag::LE));
        assert!(cpsr.is_condition_set(ConditionCodeFlag::AL));
    }

    #[test]
    fn set_interrupt_bit() {
        let mut cpsr_irq_set = Cpsr::from(0);
        let mut cpsr_irq_unset = Cpsr::from(0);
        let mut cpsr_fiq_set = Cpsr::from(0);
        let mut cpsr_fiq_unset = Cpsr::from(0);

        cpsr_irq_set.set_interrupt_bit(Interruption::Irq, BitState::Set);
        cpsr_irq_unset.set_interrupt_bit(Interruption::Irq, BitState::Unset);

        cpsr_fiq_set.set_interrupt_bit(Interruption::Fiq, BitState::Set);
        cpsr_fiq_unset.set_interrupt_bit(Interruption::Fiq, BitState::Unset);

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

        assert_eq!(cpsr_irq_set.get_interrrupt_bit_state(Interruption::Irq), BitState::Set);
        assert_eq!(cpsr_fiq_set.get_interrrupt_bit_state(Interruption::Fiq), BitState::Set);

        assert_eq!(cpsr_irq_unset.get_interrrupt_bit_state(Interruption::Irq), BitState::Unset);
        assert_eq!(cpsr_fiq_unset.get_interrrupt_bit_state(Interruption::Fiq), BitState::Unset);
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

    #[test]
    fn set_condition_bit_n() {
        let mut cpsr = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        cpsr.set_condition_bit(ConditionBit::N, BitState::Set);

        let expected_result = Cpsr::from(0b10000_00_0_0000_0000_000000_00000_00000);
        assert_eq!(cpsr, expected_result, "{:#?}, {:#?}", &cpsr, &expected_result);

        cpsr.set_condition_bit(ConditionBit::N, BitState::Unset);
        let expected_result = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        assert_eq!(cpsr, expected_result, "{:#?}, {:#?}", &cpsr, &expected_result);
    }

    #[test]
    fn set_condition_bit_z() {
        let mut cpsr = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        cpsr.set_condition_bit(ConditionBit::Z, BitState::Set);

        let expected_result = Cpsr::from(0b01000_00_0_0000_0000_000000_00000_00000);
        assert_eq!(cpsr, expected_result, "{:#?}, {:#?}", &cpsr, &expected_result);

        cpsr.set_condition_bit(ConditionBit::Z, BitState::Unset);
        let expected_result = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        assert_eq!(cpsr, expected_result, "{:#?}, {:#?}", &cpsr, &expected_result);
    }

    #[test]
    fn set_condition_bit_c() {
        let mut cpsr = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        cpsr.set_condition_bit(ConditionBit::C, BitState::Set);

        let expected_result = Cpsr::from(0b00100_00_0_0000_0000_000000_00000_00000);
        assert_eq!(cpsr, expected_result, "{:#?}, {:#?}", &cpsr, &expected_result);

        cpsr.set_condition_bit(ConditionBit::C, BitState::Unset);
        let expected_result = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        assert_eq!(cpsr, expected_result, "{:#?}, {:#?}", &cpsr, &expected_result);
    }

    #[test]
    fn set_condition_bit_v() {
        let mut cpsr = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        cpsr.set_condition_bit(ConditionBit::V, BitState::Set);

        let expected_result = Cpsr::from(0b00010_00_0_0000_0000_000000_00000_00000);
        assert_eq!(cpsr, expected_result, "{:#?}, {:#?}", &cpsr, &expected_result);

        cpsr.set_condition_bit(ConditionBit::V, BitState::Unset);
        let expected_result = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        assert_eq!(cpsr, expected_result, "{:#?}, {:#?}", &cpsr, &expected_result);
    }
}
