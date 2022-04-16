use crate::ram::Word;

use super::ArmOpcode;

pub fn handle(value: Word) -> ArmOpcode {
    if is_miscellaneous_instruction(value) {
        handle_miscellaneous_instruction(value)
    } else if is_data_processing(value) {
        ArmOpcode::get_data_processing(value)
    } else if is_multiply_instruction(value) {
        handle_multiply_instruction(value)
    } else if is_extra_load_store_instruction(value) {
        handle_extra_load_store_instruction(value)
    } else {
        unreachable!();
    }
}

// or in other words: Miscellaneous
fn handle_miscellaneous_instruction(value: Word) -> ArmOpcode {
    let bit22 = (value >> 22) & 0b1;
    let bit21 = (value >> 21) & 0b1;
    let bit7 = (value >> 7) & 0b1;
    let bit6 = (value >> 6) & 0b1;
    let bit5 = (value >> 5) & 0b1;
    let bit4 = (value >> 4) & 0b1;

    match (bit22, bit21, bit7, bit6, bit5, bit4) {
        (_, 0, 0, 0, 0, 0) => ArmOpcode::MRS,
        (_, 1, 0, 0, 0, 0) => ArmOpcode::MSR,
        (0, 1, 0, 0, 0, 1) => ArmOpcode::BX,
        (1, 1, 0, 0, 0, 1) => ArmOpcode::CLZ,
        (0, 1, 0, 0, 1, 1) => ArmOpcode::BLX,
        (0, 0, 0, 1, 0, 1) => ArmOpcode::QADD,
        (0, 1, 0, 1, 0, 1) => ArmOpcode::QSUB,
        (1, 0, 0, 1, 0, 1) => ArmOpcode::QDADD,
        (1, 1, 0, 1, 0, 1) => ArmOpcode::QDSUB,
        (0, 1, 0, 1, 1, 1) => ArmOpcode::BKPT,
        (0, 0, 1, _, _, 0) => ArmOpcode::SMLAXY,
        (0, 1, 1, _, 0, 0) => ArmOpcode::SMLAWY,
        (0, 1, 1, _, 1, 0) => ArmOpcode::SMULWY,
        (1, 0, 1, _, _, 0) => ArmOpcode::SMLALXY,
        (1, 1, 1, _, _, 0) => ArmOpcode::SMULXY,
        (_, _, _, _, _, _) => unreachable!(),
    }
}

fn handle_multiply_instruction(value: Word) -> ArmOpcode {
    let bit23 = (value >> 23) & 0b1;
    let bit22 = (value >> 22) & 0b1;
    let bit21 = (value >> 21) & 0b1;
    let bit20 = (value >> 20) & 0b1;

    if bit23 == 0 && bit22 == 0 {
        match (bit21, bit20) {
            (0, _) => ArmOpcode::MUL,
            (1, _) => ArmOpcode::MLA,
            _ => unreachable!(),
        }
    } else if bit23 == 1 {
        match (bit22, bit21, bit20) {
            (0, 0, _) => ArmOpcode::UMULL,
            (0, 1, _) => ArmOpcode::UMLAL,
            (1, 0, _) => ArmOpcode::SMULL,
            (1, 1, _) => ArmOpcode::SMLAL,
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    }
}

fn handle_extra_load_store_instruction(value: Word) -> ArmOpcode {
    let bit24 = (value >> 24) & 0b1;
    let bit23 = (value >> 23) & 0b1;
    let bit22 = (value >> 22) & 0b1;
    let bit21 = (value >> 21) & 0b1;
    let bit20 = (value >> 20) & 0b1;
    let bit6 = (value >> 6) & 0b1;
    let bit5 = (value >> 5) & 0b1;

    match (bit24, bit23, bit22, bit21, bit20, bit6, bit5) {
        (1, 0, 0, 0, 0, 0, 0) => ArmOpcode::SWP,
        (1, 0, 1, 0, 0, 0, 0) => ArmOpcode::SWPB,
        (_, _, _, _, 0, 0, 1) => ArmOpcode::STRH,
        (_, _, _, _, 0, 1, 0) => ArmOpcode::LDRD,
        (_, _, _, _, 0, 1, 1) => ArmOpcode::STRD,
        (_, _, _, _, 1, 0, 1) => ArmOpcode::LDRH,
        (_, _, _, _, 1, 1, 0) => ArmOpcode::LDRSB,
        (_, _, _, _, 1, 1, 1) => ArmOpcode::LDRSH,
        (_, _, _, _, _, _, _) => unreachable!(),
    }
}

fn is_data_processing(value: Word) -> bool {
    let bit4 = (value >> 4) & 0b1;
    let bit7 = (value >> 7) & 0b1;

    bit4 == 0 || (bit7 == 0 && bit4 == 1)
}

// see page 143
fn is_multiply_instruction(value: Word) -> bool {
    let bit31_28 = (value >> 28) & 0b1111;
    let bit27_24 = (value >> 24) & 0b1111;
    let bit7_4 = (value >> 4) & 0b1111;

    bit31_28 != 0b1111 && bit27_24 == 0b0000 && bit7_4 == 0b1001
}

fn is_extra_load_store_instruction(value: Word) -> bool {
    let bit31_28 = (value >> 28) & 0b1111;
    let bit27_25 = (value >> 25) & 0b111;
    let bit7 = (value >> 7) & 0b1;
    let bit4 = (value >> 4) & 0b1;

    bit31_28 != 0b1111 && bit27_25 == 0b000 && bit7 == 1 && bit4 == 1
}

fn is_miscellaneous_instruction(value: Word) -> bool {
    let bit24_23 = (value >> 23) & 0b11;
    let bit20 = (value >> 20) & 0b1;
    let bit7 = (value >> 7) & 0b1;
    let bit4 = (value >> 4) & 0b1;

    bit24_23 == 0b10 && bit20 == 0 && (bit4 == 0 || (bit7 == 0 && bit4 == 1))
}
