use crate::ram::Word;

use super::ArmOpcode;

pub fn handle(value: Word) -> ArmOpcode {
    let bit24 = (value >> 24) & 0b1;
    let bit22 = (value >> 22) & 0b1;
    let bit21 = (value >> 21) & 0b1;
    let bit20 = (value >> 20) & 0b1;

    match (bit24, bit22, bit21, bit20) {
        (0, 0, 1, 1) => ArmOpcode::LDRT,
        (0, 1, 1, 1) => ArmOpcode::LDRBT,
        (0, 0, 1, 0) => ArmOpcode::STRT,
        (0, 1, 1, 0) => ArmOpcode::STRBT,
        (_, 0, _, 1) => ArmOpcode::LDR,
        (_, 1, _, 1) => ArmOpcode::LDRB,
        (_, 0, _, 0) => ArmOpcode::STR,
        (_, 1, _, 0) => ArmOpcode::STRB,
        _ => unreachable!(),
    }
}
