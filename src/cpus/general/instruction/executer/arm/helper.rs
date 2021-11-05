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

    /// Manual: Page 1134
    pub fn sign_extend(value: u32, sign_index: u8) -> u32 {
        let new_ones = u32::MAX & !((1 << (sign_index + 1)) - 1);
        value | new_ones
    }

    /// Manual: Page 1134
    pub fn signed_sat(x: i32, n: u32) -> i32 {
        let border_val = 2_i32.pow(n - 1);
        if x < -border_val {
            -border_val
        } else if -border_val <= x && x <= border_val - 1 {
            x
        } else {
            border_val
        }
    }

    // Manual: Page 1134
    pub fn signed_does_sat(x: i32, n: u32) -> bool {
        let border_val = 2_i32.pow(n - 1);
        !(-border_val <= x && x <= border_val -1)
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
    fn sign_extend() {
        let value = Helper::sign_extend(0b1000_0000, 7);
        let expected_value = 0b1111_1111_1111_1111_1111_1111_1000_0000;
        assert_eq!(value, expected_value, "{:b} {:b}", &value, &expected_value);
    }

    #[test]
    fn signed_sat() {
        // if x < -border_val
        let val1 = Helper::signed_sat(-9, 4);
        assert_eq!(val1, -8, "{:b}", &val1);

        // else if -border_val <= x && x <= border_val - 1
        let val2 = Helper::signed_sat(1, 3);
        assert_eq!(val2, 1, "{:b}", &val2);

        // else
        let val3 = Helper::signed_sat(9, 4);
        assert_eq!(val3, 8, "{:b}", &val3);
    }

    #[test]
    fn signed_does_sat() {
        assert!(!Helper::signed_does_sat(1, 3));
        assert!(Helper::signed_does_sat(42, 2));
    }
}
