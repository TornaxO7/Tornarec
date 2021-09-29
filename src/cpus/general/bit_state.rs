#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitState {Set, Unset}

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
}

impl From<u32> for BitState {
    fn from(bit: u32) -> Self {
        if bit > 0 {
            Self::Set
        } else {
            Self::Unset
        }
    }
}

#[cfg(test)]
mod tests {

    use super::BitState;

    #[test]
    fn from_u32() {
        let unset_val = 0;
        let val1 = 1;
        let val2 = 10;
        let val3 = 100;

        assert_eq!(BitState::from(unset_val), BitState::Unset);
        assert_eq!(BitState::from(val1), BitState::Set);
        assert_eq!(BitState::from(val2), BitState::Set);
        assert_eq!(BitState::from(val3), BitState::Set);
    }

    #[test]
    fn get_state() {
        let unset_val = 0;
        let val1 = 10;
        let val2 = 100;

        let unset_state = BitState::from(unset_val);
        let state1 = BitState::from(val1);
        let state2 = BitState::from(val2);

        assert!(unset_state.is_unset());
        assert!(state1.is_set());
        assert!(state2.is_set());
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
