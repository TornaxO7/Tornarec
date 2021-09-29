use core::ops::Add;
use std::fmt::Display;
use core::convert::{From, TryFrom};

#[derive(thiserror::Error, Clone, Debug, PartialEq, Eq)]
pub enum GeneralRegisterError<T: Display> {
    #[error("[GENERAL REGISTER ERROR]: Couldn't convert the value '{0}' to a u32.")]
    ConvertToU32(T),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GeneralRegister(u32);

impl GeneralRegister {
    pub fn set_value(&mut self, new_val: u32) {
        self.0 = new_val;
    }

    pub fn get_as_u32(&self) -> u32 {
        self.0
    }

    pub fn get_as_usize(&self) -> usize {
        match usize::try_from(self.0) {
            Ok(num) => num,
            Err(_) => panic!("{}", GeneralRegisterError::ConvertToU32(self.0)),
        }
    }
}

impl From<u32> for GeneralRegister {
    fn from(num: u32) -> Self {
        Self(num)
    }
}

impl Add<u32> for GeneralRegister {
    type Output = u32;

    fn add(self, number: u32) -> u32 {
        self.0 + number
    }
}

impl Add<Self> for GeneralRegister {
    type Output = Self;

    fn add(self, register: GeneralRegister) -> Self {
        Self(self.0 + register.0)
    }
}

#[cfg(test)]
mod tests {

    use super::GeneralRegister;

    #[test]
    fn add_u32() {
        let reg = GeneralRegister::from(10);
        assert_eq!(reg + 10, 20);
    }
    
    #[test]
    fn add_self() {
        let reg1 = GeneralRegister::from(10);
        let reg2 = GeneralRegister::from(10);
        assert_eq!(reg1 + reg2, GeneralRegister(20));
    }
}
