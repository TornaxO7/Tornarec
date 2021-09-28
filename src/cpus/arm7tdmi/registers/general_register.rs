use core::convert::TryFrom;
use core::ops::Add;
use std::fmt::Display;

#[derive(thiserror::Error, Clone, Debug, PartialEq, Eq)]
pub enum GeneralRegisterError<T: Display> {
    #[error("[GENERAL REGISTER ERROR]: Couldn't convert the value '{0}' to a u32.")]
    ConvertToU32(T),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GeneralRegister(u32);

impl GeneralRegister {
    pub fn get_as_usize(&self) -> usize {
        match usize::try_from(self.0) {
            Ok(num) => num,
            Err(_) => panic!("{}", GeneralRegisterError::ConvertToU32(self.0)),
        }
    }
}

impl Add<u32> for GeneralRegister {
    type Output = Self;

    fn add(self, number: u32) -> Self {
        Self(self.0 + number)
    }
}

impl Add<Self> for GeneralRegister {
    type Output = Self;

    fn add(self, register: GeneralRegister) -> Self {
        Self(self.0 + register.0)
    }
}
