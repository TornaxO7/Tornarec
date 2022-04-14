use crate::{
    cpus::general::{
        condition_code_flag::ConditionCodeFlag,
        instruction::arm::{
            opcode::ArmOpcode,
            operand::ArmOperand,
            ArmInstruction, BitState,
        },
    },
    ram::{
        Address,
        Word,
    },
};

mod miscellaneous;
mod multiplies;

pub fn handle000(address: Address, value: Word) -> ArmInstruction {
    let bit24 = (value >> 23) & 0b1;
    let bit23 = (value >> 23) & 0b1;
    let bit20 = (value >> 20) & 0b1;
    let bit7 = (value >> 7) & 0b1;
    let bit4 = (value >> 4) & 0b1;

    match (bit24, bit23, bit20, bit7, bit4) {
        (_, _, _, _, 0) | (_, _, _, 0, 1) => get_data_processing(address, value),
        (1, 0, 0, 0, 1) | (1, 0, 0, _, 0) => get_miscellaneous_instruction(address, value),
        (_, _, _, 1, 1) => multiplies_and_extra_load_store(address, value),
        (_, _, _, _, _) => todo!("Unknown [000] instruction: {:#034b}", value),
    }
}

/// ARM INSTRUCTIONS
fn get_data_processing(address: Address, value: Word) -> ArmInstruction {
    ArmInstruction {
        opcode: ArmOpcode::get_data_processing_operand(value),
        operand: ArmOperand::get_addressing_mode1(value),
        address,
        cond: ConditionCodeFlag::from(value),
    }
}

fn get_miscellaneous_instruction(address: Address, value: Word) -> ArmInstruction {
    let bit27_23 = (value >> 23) & 0b1_1111;

    if bit27_23 == 0b0_0110 {
        return miscellaneous::get_msr(address, value);
    };

    let bit22 = (value >> 22) & 0b1;
    let bit21 = (value >> 21) & 0b1;
    let bit20 = (value >> 20) & 0b1;

    let bit7 = (value >> 7) & 0b1;
    let bit6 = (value >> 6) & 0b1;
    let bit5 = (value >> 5) & 0b1;
    let bit4 = (value >> 4) & 0b1;
    match (bit22, bit21, bit20, bit7, bit6, bit5, bit4) {
        (_, 0, 0, 0, 0, 0, 0) => miscellaneous::get_mrs(address, value),
        (_, 1, 0, 0, 0, 0, 0) => miscellaneous::get_msr(address, value),
        (0, 1, 0, 0, 0, 0, 1) => miscellaneous::get_bx(address, value),
        (0, 1, 0, 0, 0, 1, 0) => todo!("[Need BJX] Figure A3-4 (page 145)"),
        (1, 1, 0, 0, 0, 0, 1) => miscellaneous::get_clz(address, value),
        (0, 1, 0, 0, 0, 1, 1) => miscellaneous::get_blx(address, value),
        (_, _, 0, 0, 1, 0, 1) => miscellaneous::get_saturating_add_subtract(address, value),
        (0, 1, 0, 0, 1, 1, 1) => miscellaneous::get_bkpt(address, value),
        (_, _, 0, 1, _, _, 0) => miscellaneous::get_signed_multiplies_type2(address, value),
        (_, _, _, _, _, _, _) => unreachable!("[Miscellaneous] Unknown opcode: {:#034b}", value),
    }
}

fn multiplies_and_extra_load_store(address: Address, value: Word) -> ArmInstruction {
    let bit23 = BitState::from(((value >> 23) & 0b1) != 0);

    if bit23 {
        multiplies::get_multiply_long(address, value)
    } else {
        multiplies::get_multiply(address, value)
    }
}
