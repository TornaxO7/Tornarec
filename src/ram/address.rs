use core::fmt;
use core::fmt::{UpperHex, LowerHex};
use core::ops::Add;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Address(usize);

impl Address {
    pub fn get_ref(&self) -> &usize {
        &self.0
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

impl From<usize> for Address {
    fn from(num: usize) -> Self {
        Self(num)
    }
}

impl From<u32> for Address {
    fn from(num: u32) -> Self {
        Self(num as usize)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl UpperHex for Address {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:X}", self.0)
    }
}

impl LowerHex for Address {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:x}", self.0)
    }
}

impl Add<usize> for Address {
    type Output = Self;

    fn add(self, number: usize) -> Self {
        Self(self.0 + number)
    }
}

impl Add<Self> for Address {
    type Output = Self;

    fn add(self, address: Address) -> Self {
        Self(self.0 + address.0)
    }
}

#[cfg(test)]
mod tests {

    use super::Address;

    #[test]
    fn get_ref() {
        let input = 42;
        let address = Address(input);
        assert_eq!(address.get_ref(), &input);
    }
    
    #[test]
    fn get() {
        let input = 42;
        let address = Address(input);
        assert_eq!(address.get(), input);
    }

    #[test]
    fn from_usize() {
        let input: usize = 42;
        let address = Address::from(input);
        assert_eq!(address, Address(input));
    }

    #[test]
    fn display() {
        let address = Address(42);
        assert_eq!(format!("{}", address), "42".to_string());
    }

    #[test]
    fn upper_hex() {
        let address = Address(42);
        assert_eq!(format!("{:X}", address), "2A");
    }

    #[test]
    fn lower_hex() {
        let address = Address(42);
        assert_eq!(format!("{:x}", address), "2a");
    }

    #[test]
    fn add_with_usize() {
        let address = Address(22);
        let number: usize = 20;

        assert_eq!(address + number, Address(42));
    }

    #[test]
    fn add_two_addresses() {
        let address1 = Address(16);
        let address2 = Address(26);

        assert_eq!(address1 + address2, Address(42));
    }
}
