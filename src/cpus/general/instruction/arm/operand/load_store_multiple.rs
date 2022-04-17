use crate::{ram::Word, cpus::general::instruction::arm::{BitState, types::{RegisterList, Register}}};

use super::ArmOperand;

pub fn get_ldm_stm_operand(value: Word) -> ArmOperand {
    ArmOperand::LDMandSTM {
        s: BitState::new(value, 22),
        w: BitState::new(value, 21),
        rn: Register::new(value, 16, 0b1111),
        register_list: RegisterList::new(value, 0, 0b1111_1111_1111_111),
        mode: LoadStoreMultipleMode::from(value),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadStoreMultipleMode {
    IncrementAfter,
    IncrementBefore,
    DecrementAfter,
    DecrementBefore,
}

impl From<Word> for LoadStoreMultipleMode {
    fn from(value: Word) -> Self {
        let p = BitState::new(value, 24);
        let u = BitState::new(value, 23);

        match (p, u) {
            (BitState::UNSET, BitState::UNSET) => Self::DecrementAfter,
            (BitState::UNSET, BitState::SET) => Self::IncrementAfter,
            (BitState::SET, BitState::UNSET) => Self::DecrementBefore,
            (BitState::SET, BitState::SET) => Self::IncrementBefore,
        }
    }
}
