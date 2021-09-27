#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {Set, Unset}

impl From<u32> for State {
    fn from(bit: u32) -> Self {
        if bit > 0 {
            Self::Set
        } else {
            Self::Unset
        }
    }
}

