use crate::cpus::general::{
    condition_code_flag::ConditionCodeFlag,
    instruction::types::BitState,
    interruption::Interruption,
    operating_mode::OperatingMode,
    operating_state::OperatingState,
    register::types::{
        ConditionBit,
        ConditionBits,
    },
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
        ![OperatingMode::Usr, OperatingMode::Sys].contains(&self.get_operating_mode().unwrap())
    }

    pub fn in_privileged_mode(&self) -> bool {
        match self.get_operating_mode().unwrap() {
            OperatingMode::Usr => false,
            _ => true,
        }
    }

    pub fn is_condition_set(&self, condition: ConditionCodeFlag) -> bool {
        let cb = self.get_condition_bits();

        match condition {
            ConditionCodeFlag::EQ => *cb.z,
            ConditionCodeFlag::NE => !cb.z,
            ConditionCodeFlag::CS => *cb.c,
            ConditionCodeFlag::CC => !cb.c,
            ConditionCodeFlag::MI => *cb.n,
            ConditionCodeFlag::PL => !cb.n,
            ConditionCodeFlag::VS => *cb.v,
            ConditionCodeFlag::VC => !cb.v,
            ConditionCodeFlag::HI => *cb.c && !cb.z,
            ConditionCodeFlag::LS => !cb.c && *cb.z,
            ConditionCodeFlag::GE => *cb.n == *cb.v,
            ConditionCodeFlag::LT => *cb.n != *cb.v,
            ConditionCodeFlag::GT => !cb.z && (cb.n == cb.v),
            ConditionCodeFlag::LE => *cb.z || (cb.n != cb.v),
            ConditionCodeFlag::AL => true,
        }
    }

    pub fn get_condition_bits(&self) -> ConditionBits {
        ConditionBits::from(self.0)
    }

    pub fn set_condition_bits(&mut self, condition_bits: ConditionBits) {
        self.set_condition_bit(ConditionBit::N, condition_bits.n);
        self.set_condition_bit(ConditionBit::Z, condition_bits.z);
        self.set_condition_bit(ConditionBit::C, condition_bits.c);
        self.set_condition_bit(ConditionBit::V, condition_bits.v);
        self.set_condition_bit(ConditionBit::Q, condition_bits.q);
    }

    pub fn set_interrupt_bit(&mut self, interrupt: Interruption, state: BitState) {
        match interrupt {
            Interruption::Irq => match state {
                BitState::UNSET => self.0 &= !(1 << 7),
                BitState::SET => self.0 |= 1 << 7,
            },
            Interruption::Fiq => match state {
                BitState::UNSET => self.0 &= !(1 << 6),
                BitState::SET => self.0 |= 1 << 6,
            },
        }
    }

    pub fn get_interrrupt_bit_state(&self, interrupt: Interruption) -> BitState {
        match interrupt {
            Interruption::Irq => {
                if (self.0 >> 7) & 1 == 1 {
                    BitState::SET
                } else {
                    BitState::UNSET
                }
            }
            Interruption::Fiq => {
                if (self.0 >> 6) & 1 == 1 {
                    BitState::SET
                } else {
                    BitState::UNSET
                }
            }
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
        let state_val = u32::from(state);
        match condition_bit {
            ConditionBit::N => self.0 = (self.0 & !(1 << 31)) | (state_val << 31),
            ConditionBit::Z => self.0 = (self.0 & !(1 << 30)) | (state_val << 30),
            ConditionBit::C => self.0 = (self.0 & !(1 << 29)) | (state_val << 29),
            ConditionBit::V => self.0 = (self.0 & !(1 << 28)) | (state_val << 28),
            ConditionBit::Q => self.0 = (self.0 & !(1 << 27)) | (state_val << 27),
        };
    }

    pub fn get_condition_bit(&self, condition_bit: ConditionBit) -> BitState {
        match condition_bit {
            ConditionBit::N => BitState::new(self.0, 31),
            ConditionBit::Z => BitState::new(self.0, 30),
            ConditionBit::C => BitState::new(self.0, 29),
            ConditionBit::V => BitState::new(self.0, 28),
            ConditionBit::Q => BitState::new(self.0, 27),
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

    use super::{
        BitState,
        ConditionBit,
        ConditionCodeFlag,
        Cpsr,
        CpsrError,
        Interruption,
        OperatingMode,
        OperatingState,
    };

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

        cpsr_irq_set.set_interrupt_bit(Interruption::Irq, BitState::SET);
        cpsr_irq_unset.set_interrupt_bit(Interruption::Irq, BitState::UNSET);

        cpsr_fiq_set.set_interrupt_bit(Interruption::Fiq, BitState::SET);
        cpsr_fiq_unset.set_interrupt_bit(Interruption::Fiq, BitState::UNSET);

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

        assert_eq!(
            cpsr_irq_set.get_interrrupt_bit_state(Interruption::Irq),
            BitState::SET
        );
        assert_eq!(
            cpsr_fiq_set.get_interrrupt_bit_state(Interruption::Fiq),
            BitState::SET
        );

        assert_eq!(
            cpsr_irq_unset.get_interrrupt_bit_state(Interruption::Irq),
            BitState::UNSET
        );
        assert_eq!(
            cpsr_fiq_unset.get_interrrupt_bit_state(Interruption::Fiq),
            BitState::UNSET
        );
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
        // it shouldn't matter which previous it had, so we're setting the mode with
        // some random bits
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
        assert_eq!(
            cpsr_unknown.get_operating_mode(),
            Err(CpsrError::UnknownOperatingMode(0b10101))
        );
    }

    #[test]
    fn set_condition_bit_n() {
        let mut cpsr = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        cpsr.set_condition_bit(ConditionBit::N, BitState::SET);

        let expected_result = Cpsr::from(0b10000_00_0_0000_0000_000000_00000_00000);
        assert_eq!(
            cpsr, expected_result,
            "{:#?}, {:#?}",
            &cpsr, &expected_result
        );

        cpsr.set_condition_bit(ConditionBit::N, BitState::UNSET);
        let expected_result = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        assert_eq!(
            cpsr, expected_result,
            "{:#?}, {:#?}",
            &cpsr, &expected_result
        );
    }

    #[test]
    fn set_condition_bit_z() {
        let mut cpsr = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        cpsr.set_condition_bit(ConditionBit::Z, BitState::SET);

        let expected_result = Cpsr::from(0b01000_00_0_0000_0000_000000_00000_00000);
        assert_eq!(
            cpsr, expected_result,
            "{:#?}, {:#?}",
            &cpsr, &expected_result
        );

        cpsr.set_condition_bit(ConditionBit::Z, BitState::UNSET);
        let expected_result = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        assert_eq!(
            cpsr, expected_result,
            "{:#?}, {:#?}",
            &cpsr, &expected_result
        );
    }

    #[test]
    fn set_condition_bit_c() {
        let mut cpsr = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        cpsr.set_condition_bit(ConditionBit::C, BitState::SET);

        let expected_result = Cpsr::from(0b00100_00_0_0000_0000_000000_00000_00000);
        assert_eq!(
            cpsr, expected_result,
            "{:#?}, {:#?}",
            &cpsr, &expected_result
        );

        cpsr.set_condition_bit(ConditionBit::C, BitState::UNSET);
        let expected_result = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        assert_eq!(
            cpsr, expected_result,
            "{:#?}, {:#?}",
            &cpsr, &expected_result
        );
    }

    #[test]
    fn set_condition_bit_v() {
        let mut cpsr = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        cpsr.set_condition_bit(ConditionBit::V, BitState::SET);

        let expected_result = Cpsr::from(0b00010_00_0_0000_0000_000000_00000_00000);
        assert_eq!(
            cpsr, expected_result,
            "{:#?}, {:#?}",
            &cpsr, &expected_result
        );

        cpsr.set_condition_bit(ConditionBit::V, BitState::UNSET);
        let expected_result = Cpsr::from(0b00000_00_0_0000_0000_000000_00000_00000);
        assert_eq!(
            cpsr, expected_result,
            "{:#?}, {:#?}",
            &cpsr, &expected_result
        );
    }
}
