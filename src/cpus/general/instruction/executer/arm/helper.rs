use crate::cpus::general::BitState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Helper;

impl Helper {
    pub fn carry_from(values: Vec<u32>) -> BitState {
        let mut sum: u32 = 0;
        for value in values {
            match sum.checked_add(value) {
                Some(new_sum) => sum = new_sum,
                None => return BitState::Set,
            }
        }
        BitState::Unset
    }

    pub fn borrow_from(values: Vec<u32>) -> BitState {
        let mut num = values[0].clone();
        for value in values[1..].iter() {
            match num.checked_sub(*value) {
                Some(new_num) => num = new_num,
                None => return BitState::Set,
            }
        }
        BitState::Unset
    }

    pub fn overflow_from(values: Vec<i32>) -> BitState {
        let mut sum: i32 = 0;
        for value in values {
            match sum.checked_add(value) {
                Some(new_sum) => sum = new_sum,
                None => return BitState::Set,
            }
        }
        BitState::Unset
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitState,
        Helper,
    };

    #[test]
    fn carry() {
        let values = vec![0xFFFF_FFFF, 1];
        assert_eq!(Helper::carry_from(values), BitState::Set);

        let values = vec![1, 1];
        assert_eq!(Helper::carry_from(values), BitState::Unset);
    }

    #[test]
    fn borrow() {
        let values = vec![0x0, 0x1];
        assert_eq!(Helper::borrow_from(values), BitState::Set);

        let values = vec![0x1, 0x1];
        assert_eq!(Helper::borrow_from(values), BitState::Unset);
    }

    #[test]
    fn overflow() {
        let values = vec![i32::MAX, 1];
        assert_eq!(Helper::overflow_from(values), BitState::Set);

        let values = vec![0, 1];
        assert_eq!(Helper::overflow_from(values), BitState::Unset);
    }
}
