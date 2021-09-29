pub mod exception_stack;

pub use exception_stack::ExceptionStack;

use crate::ram::Address;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exception {
    Swi,
    Udef,
    Pabt,
    Fiq,
    Irq,
    Dabt,
    Reset,
}

impl Exception {
    pub fn get_exception_vector(&self) -> Address {
        match self {
            Exception::Swi   => Address::from(0x00000008),
            Exception::Udef  => Address::from(0x00000004),
            Exception::Pabt  => Address::from(0x0000000C),
            Exception::Fiq   => Address::from(0x0000001C),
            Exception::Irq   => Address::from(0x00000018),
            Exception::Dabt  => Address::from(0x00000010),
            Exception::Reset => Address::from(0x00000000),
        }
    }

    pub fn get_priority(&self) -> u8 {
        match self {
            Exception::Reset => 1,
            Exception::Dabt  => 2,
            Exception::Fiq   => 3,
            Exception::Irq   => 4,
            Exception::Pabt  => 5,
            Exception::Udef | Exception::Swi => 6,
        }
    }
}
