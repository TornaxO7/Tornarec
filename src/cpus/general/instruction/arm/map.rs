use crate::{ram::{Address, Word}, cpus::general::register::Registers};

use super::{ArmInstruction, BitState};

pub fn get_arm_instruction(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {
    let bit25_27 = (value >> 25) & 0b111;

    match bit25_27 {
        0b000 => handle000(address, value, registers),
        0b001 => handle001(address, value, registers),
        0b010 => handle010(address, value, registers),
        0b011 => handle011(address, value, registers),
        0b100 => handle100(address, value, registers),
        0b101 => handle101(address, value, registers),
        0b110 => handle110(address, value, registers),
        0b111 => handle111(address, value, registers),
    }
}

// opcode[25:27] = 0b000
fn handle000(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {
    let bit24 = (value >> 23) & 0b1;
    let bit23 = (value >> 23) & 0b1;
    let bit20 = (value >> 20) & 0b1;
    let bit7 = (value >> 7) & 0b1;
    let bit4 = (value >> 4) & 0b1;

    match (bit24, bit23, bit20, bit7, bit4) {
        (_, _, _, _, 0) => data_processing_immediate_shift(address, value, registers),
        (1, 0, 0, _, 0) => miscellaneous_instructions1(address, value, registers),
        (_, _, _, 0, 1) => data_processing_register_shift(address, value, registers),
        (1, 0, 0, 0, 1) => miscellaneous_instructions2(address, value, registers),
        (_, _, _, 1, 1) => multiplies_and_extra_load_store(address, value, registers),
    }
}

// opcode[25:27] = 0b001
fn handle001(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {
}

// opcode[25:27] = 0b010
fn handle010(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {
}

fn handle011(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {
}

fn handle100(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {
}

fn handle101(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {
}

fn handle110(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {
}

fn handle111(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {
}

/// ARM INSTRUCTIONS
fn data_processing_immediate_shift(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {
}

fn miscellaneous_instructions1(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {
}

fn data_processing_register_shift(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {
}

fn miscellaneous_instructions2(address: &Address, value: &Word, registers: &Registers) -> ArmInstruction {
}

fn multiplies_and_extra_load_store(address: &Address, value: &Word, regisers: &Registers) -> ArmInstruction {
}
