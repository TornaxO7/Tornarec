use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Register(u8);

impl Deref for Register {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
