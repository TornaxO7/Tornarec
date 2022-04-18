use crate::{
    cpus::general::instruction::arm::{
        types::{
            sbo,
            Register,
        },
        BitState,
    },
    ram::Word,
};

use super::ArmOperand;

pub fn get_immed24(value: Word) -> ArmOperand {
    ArmOperand::Branch(BranchOperand::get_immediate(value))
}

pub fn get_link_exchange_immed(value: Word) -> ArmOperand {
    ArmOperand::Branch(BranchOperand::get_link_exchange_immediate(value))
}

pub fn get_register(value: Word) -> ArmOperand {
    ArmOperand::Branch(BranchOperand::get_register(value))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BranchOperand {
    Immediate { signed_immed_24: u32 },
    LinkExchangeImmed { h: BitState, signed_immed_24: u32 },
    Register { rm: Register },
}

impl BranchOperand {
    pub fn get_immediate(value: Word) -> Self {
        Self::Immediate {
            signed_immed_24: value & 0b1111_1111_1111_1111_1111_1111,
        }
    }

    pub fn get_link_exchange_immediate(value: Word) -> Self {
        Self::LinkExchangeImmed {
            h: BitState::new(value, 24),
            signed_immed_24: value & 0b1111_1111_1111_1111_1111_1111,
        }
    }

    pub fn get_register(value: Word) -> Self {
        sbo(value, 8, 0b1111_1111_1111);
        Self::Register {
            rm: Register::new(value, 0, 0b1111),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{
        BitState,
        BranchOperand,
        Register,
    };

    #[test]
    fn test_get_immediate() {
        let value = 0b0000_1011_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            BranchOperand::Immediate {
                signed_immed_24: (1 << 24) - 1,
            },
            BranchOperand::get_immediate(value)
        );
    }

    #[test]
    fn test_get_link_exchange_immediate() {
        let value = 0b1111_1011_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            BranchOperand::LinkExchangeImmed {
                h: BitState::SET,
                signed_immed_24: (1 << 24) - 1,
            },
            BranchOperand::get_link_exchange_immediate(value)
        );
    }

    #[test]
    fn test_get_register() {
        let value = 0b0000_0001_0010_1111_1111_1111_0011_1111;

        assert_eq!(
            BranchOperand::Register {
                rm: Register::from(0b1111),
            },
            BranchOperand::get_register(value)
        );
    }

    #[test]
    #[should_panic]
    fn test_get_link_exchange_register_sbo() {
        let value = 0b0000_0001_0010_0000_0000_0000_0011_1111;
        BranchOperand::get_register(value);
    }
}
