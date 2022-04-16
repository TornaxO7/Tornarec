use crate::{
    cpus::general::instruction::arm::BitState,
    ram::Word,
};

use super::ArmOpcode;

pub fn handle(value: Word) -> ArmOpcode {
    if is_software_interrupt(value) {
        ArmOpcode::SWI
    } else if is_coprocessor_data_processing(value) {
        ArmOpcode::CDP
    } else if is_coprocessor_register_transfers(value) {
        handle_coprocessor_register_transfers(value)
    } else {
        unreachable!()
    }
}

fn handle_coprocessor_register_transfers(value: Word) -> ArmOpcode {
    let bit20 = BitState::from(((value >> 20) & 0b1) != 0);

    match bit20 {
        true => ArmOpcode::MRC,
        false => ArmOpcode::MCR,
    }
}

fn is_software_interrupt(value: Word) -> bool {
    let bit24 = BitState::from(((value >> 24) & 0b1) != 0);
    bit24
}

fn is_coprocessor_data_processing(value: Word) -> bool {
    let bit24 = BitState::from(((value >> 24) & 0b1) != 0);
    let bit4 = BitState::from(((value >> 4) & 0b1) != 0);

    !bit24 && !bit4
}

fn is_coprocessor_register_transfers(value: Word) -> bool {
    let bit24 = BitState::from(((value >> 24) & 0b1) != 0);
    let bit4 = BitState::from(((value >> 4) & 0b1) != 0);

    !bit24 && bit4
}
