mod bit_state;
mod register;
mod register_list;

pub use bit_state::BitState;
pub use register::Register;
pub use register_list::RegisterList;

use crate::ram::Word;

pub type Shift = u8;
pub type Mask = u32;

pub fn sbo(encoding: Word, shift: u8, bit_mask: u32) {
    let field = (encoding >> shift) & bit_mask;
    assert!(field == bit_mask);
}

pub fn sbz(encoding: Word, shift: u8, bit_mask: u32) {
    let field = (encoding >> shift) & bit_mask;
    assert!(field == 0);
}

#[cfg(test)]
mod tests {
    use super::{sbo, sbz};


    #[test]
    fn test_sbo_success() {
        // the first four bits must be 1
        sbo(0b1111, 0, 0b1111);

        // the second bit must be 1
        sbo(0b0111, 1, 0b1);

        // the fourth bit to the second bit must be 1
        sbo(0b1110, 1, 0b111);
    }

    #[test]
    #[should_panic]
    fn test_sbo_no_bits_set() {
        sbo(0, 0, 0b1111);

        // third bit isn't set
        sbo(0b0111, 3, 0b1);
    }

    #[test]
    #[should_panic]
    fn test_sbo_second_bit_not_set() {
        sbo(0b1010, 1, 0b11);
    }

    #[test]
    #[should_panic]
    fn test_sbo_third_not_set() {
        sbo(0b0111, 3, 0b1);
    }

    #[test]
    fn test_sbz_success() {
        // no bits are set
        sbz(0, 0, 0b1111);

        // second bit is not set
        sbz(0b1011, 2, 0b1);

        // third and second bit are not set
        sbz(0b0011, 2, 0b11);
    }

    #[test]
    #[should_panic]
    fn test_sbz_first_bit_set() {
        sbz(0b0001, 0, 0b1);
    }

    #[test]
    fn test_sbz_third_to_second_bit_set() {
        sbz(0b0011, 2, 0b11);
    }

    #[test]
    #[should_panic]
    fn test_sbz_first_and_second_bit_set() {
        sbz(0b0110, 1, 0b11);
    }
}
