use std::{
    convert::TryFrom,
    ops::Deref,
};

use crate::{
    cpus::general::instruction::types::BitState,
    ram::Word,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegisterList(Vec<BitState>);

impl RegisterList {
    pub fn new(value: Word, shift: u32, mask: u32) -> Self {
        let value = (value >> shift) & mask;
        let amount_ones = usize::try_from(mask.count_ones()).unwrap();

        let mut list = Vec::with_capacity(usize::try_from(amount_ones).unwrap());

        for index in 0..amount_ones {
            let bit = BitState::new(value, u32::try_from(index).unwrap());
            list.push(bit);
        }

        Self(list)
    }
}

impl Deref for RegisterList {
    type Target = Vec<BitState>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for RegisterList {
    type Item = BitState;
    type IntoIter = std::vec::IntoIter<BitState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::cpus::general::instruction::arm::{
        types::RegisterList,
        BitState,
    };

    #[test]
    fn test_new() {
        // four register list
        let value = 0b1111;

        assert_eq!(
            Vec::from([BitState::SET].repeat(4)),
            *RegisterList::new(value, 0, 0b1111)
        );

        // three set
        let value = 0b1110;
        assert_eq!(
            Vec::from([BitState::SET].repeat(3)),
            *RegisterList::new(value, 1, 0b111)
        );

        // some register set und some unset
        let value = 0b1001;
        assert_eq!(
            Vec::from([
                BitState::SET,
                BitState::UNSET,
                BitState::UNSET,
                BitState::SET
            ]),
            *RegisterList::new(value, 0, 0b1111)
        );
    }
}
