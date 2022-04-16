use crate::{
    cpus::general::instruction::arm::BitState,
    ram::Word,
};

use super::ArmOpcode;

pub fn handle(value: Word) -> ArmOpcode {
    if is_media_instruction(value) {
        handle_media_instruction(value)
    } else if is_load_store_instruction(value) {
        handle_load_store(value)
    } else if is_architecturally_undefined(value) {
        unreachable!("Architecurally undefined")
    } else {
        unreachable!()
    }
}

fn handle_load_store(value: Word) -> ArmOpcode {
    // NOTE: It's the same code as in `opcode010`
    // Probably finding another solution?
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
        _ => unreachable!()
    }
}

fn handle_media_instruction(_value: Word) -> ArmOpcode {
    unreachable!("Media instruction are only supported for ARMv6");
}

fn is_load_store_instruction(value: Word) -> bool {
    let bit4 = BitState::from(((value >> 4) & 0b1) != 0);
    !bit4
}

fn is_media_instruction(value: Word) -> bool {
    let bit27_25 = (value >> 25) & 0b111;
    let bit4 = BitState::from(((value >> 4) & 0b1) != 0);

    bit27_25 == 0b011 && bit4
}

fn is_architecturally_undefined(value: Word) -> bool {
    let bit24_20 = (value >> 20) & 0b1111_1;
    let bit7_4 = (value >> 4) & 0b1111;

    bit24_20 == 0b1111_1 && bit7_4 == 0b1111
}
