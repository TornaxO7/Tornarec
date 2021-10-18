mod exception_stack;
mod exception_vector;

pub use exception_stack::ExceptionStack;
pub use exception_vector::ExceptionVector;

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
