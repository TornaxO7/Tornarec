use crate::{
    cpus::general::{
        condition_code_flag::ConditionCodeFlag,
        instruction::arm::{
            opcode::ArmOpcode,
            operand::ArmOperand,
            ArmInstruction,
            BitState,
        },
    },
    ram::{
        Address,
        Word,
    },
};

pub fn get_extra_load_store_instruction(address: Address, value: Word) -> ArmInstruction {
    if !is_load_store_instruction(value) {
        unreachable!("[Arm Instruction] Unknown extra load store instruction: {:#034b}", value);
    }

    let bit24 = (value >> 24) & 0b1;
    let bit23 = (value >> 23) & 0b1;
    let bit22 = (value >> 22) & 0b1;
    let bit21 = (value >> 21) & 0b1;
    let bit20 = (value >> 20) & 0b1;
    let bit6 = (value >> 6) & 0b1;
    let bit5 = (value >> 5) & 0b1;

    todo!();
    // match (bit24, bit23, bit22, bit21, bit20, bit6, bit5) {
    //     (1, 0, _, 0, 0, 1, 0, 0, 1) => get_swp(address, value),
    //     (1, 1, 0, 0, _, 1, 0, 0, 1) => unreachable!("[Extra load store] Used LDREX/STREX"),
    //     (_, _, 0, _, _, 1, 0, 1, 1) => get_register_halfword(address, value),
    //
    // }
}

// see page 146
pub fn is_extra_load_store_instruction(value: Word) -> bool {
    let bit31_28 = (value >> 28) & 0b1111;
    let bit27_25 = (value >> 25) & 0b111;
    let bit7 = (value >> 7) & 0b1;
    let bit4 = (value >> 4) & 0b1;

    bit31_28 != 0b1111 && bit27_25 == 0b000 && bit7 == 1 && bit4 == 1
}

fn is_load_store_instruction(value: Word) -> bool {
    let bit27_25 = (value >> 25) & 0b1111;
    let bit7 = BitState::from(((value >> 7) & 0b1) != 0);
    let bit4 = BitState::from(((value >> 4) & 0b1) != 0);

    bit27_25 == 0 && bit7 && bit4
}

fn get_swp(address: Address, value: Word) -> ArmInstruction {
    let operand = ArmOperand::get_semaphore(value);
    let cond = ConditionCodeFlag::from(value);

    let b = BitState::from(((value >> 22) & 0b1) != 0);
    let opcode = if b {
        ArmOpcode::SWPB
    } else {
        ArmOpcode::SWP
    };

    ArmInstruction {
        opcode,
        operand,
        cond,
        address,
    }
}
