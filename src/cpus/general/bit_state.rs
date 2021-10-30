use std::ops::{Deref, Not};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitState {
    Set,
    Unset,
}

impl BitState {
    pub fn is_set(&self) -> bool {
        match self {
            BitState::Set => true,
            BitState::Unset => false,
        }
    }

    pub fn is_unset(&self) -> bool {
        !self.is_set()
    }

    pub fn get_as_u32(&self) -> u32 {
        match self {
            BitState::Set => 1,
            BitState::Unset => 0,
        }
    }

    pub fn get_as_i32(&self) -> i32 {
        match self {
            BitState::Set => 1,
            BitState::Unset => 0,
        }
    }
}

impl From<u32> for BitState {
    fn from(num: u32) -> Self {
        if num & 0b1 == 0b1 {
            Self::Set
        } else {
            Self::Unset
        }
    }
}

impl From<i32> for BitState {
    fn from(num: i32) -> Self {
        if num & 0b1 == 0b1 {
            Self::Set
        } else {
            Self::Unset
        }
    }
}

impl From<u8> for BitState {
    fn from(num: u8) -> Self {
        if num & 0b1 == 0b1 {
            Self::Set
        } else {
            Self::Unset
        }
    }
}

impl From<bool> for BitState {
    fn from(val: bool) -> Self {
        match val {
            true => Self::Set,
            false => Self::Unset,
        }
    }
}

impl Default for BitState {
    fn default() -> Self {
        BitState::Set
    }
}

impl Deref for BitState {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        match self {
            BitState::Set => &1,
            BitState::Unset => &0,
        }
    }
}

impl Not for BitState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Set => Self::Unset,
            Self::Unset => Self::Set,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::BitState;

    #[test]
    fn from_num() {
        let unset_val = 0;
        let val1 = 1;
        let val2 = 10;
        let val3: u32 = 100;

        assert_eq!(BitState::from(unset_val), BitState::Unset);
        assert_eq!(BitState::from(val1), BitState::Set);
        assert_eq!(BitState::from(val2), BitState::Unset);
        assert_eq!(BitState::from(val3), BitState::Unset);
    }

    #[test]
    fn get_state() {
        let unset_val = 0;
        let val1 = 1;
        let val2 = 100;

        let unset_state = BitState::from(unset_val);
        let state1 = BitState::from(val1);
        let state2 = BitState::from(val2);

        assert!(unset_state.is_unset());
        assert!(state1.is_set());
        assert!(state2.is_unset());
    }

    #[test]
    fn equality() {
        let val1 = 10;
        let val2 = 10;

        let state1 = BitState::from(val1);
        let state2 = BitState::from(val2);

        assert!(state1 == state2);
    }
}
