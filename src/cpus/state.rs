#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {Set, Unset}

impl State {
    pub fn is_set(&self) -> bool {
        match self {
            State::Set => true,
            State::Unset => false,
        }
    }

    pub fn is_unset(&self) -> bool {
        !self.is_set()
    }
}

impl From<u32> for State {
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

    use super::State;

    #[test]
    fn from_u32() {
        let unset_val = 0;
        let val1 = 1;
        let val2 = 10;
        let val3 = 100;

        assert_eq!(State::from(unset_val), State::Unset);
        assert_eq!(State::from(val1), State::Set);
        assert_eq!(State::from(val2), State::Set);
        assert_eq!(State::from(val3), State::Set);
    }

    #[test]
    fn get_state() {
        let unset_val = 0;
        let val1 = 10;
        let val2 = 100;

        let unset_state = State::from(unset_val);
        let state1 = State::from(val1);
        let state2 = State::from(val2);

        assert!(unset_state.is_unset());
        assert!(state1.is_set());
        assert!(state2.is_set());
    }

    #[test]
    fn equality() {
        let val1 = 10;
        let val2 = 10;

        let state1 = State::from(val1);
        let state2 = State::from(val2);

        assert!(state1 == state2);
    }
}
