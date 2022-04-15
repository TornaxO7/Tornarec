use crate::ram::Word;

use super::ArmOpcode;

pub fn handle(value: Word) -> ArmOpcode {
    let bit24_23 = (value >> 23) & 0b11;
    let bit21_20 = (value >> 20) & 0b11;

    match (bit24_23, bit21_20) {
        (0b10, 0b00) => unreachable!("Undefined instruction"),
        (0b10, 0b10) => ArmOpcode::MSR,
        _ => ArmOpcode::get_data_processing(value),
    }
}
