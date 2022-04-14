//! This module contains the instructions listed in Figure A3-1
mod instruction000;
mod instruction001;
mod instruction010;
mod instruction011;
mod instruction100;
mod instruction101;
mod instruction110;
mod instruction111;

use instruction000::handle000;
use instruction001::handle001;
use instruction010::handle010;
use instruction011::handle011;
use instruction100::handle100;
use instruction101::handle101;
use instruction110::handle110;
use instruction111::handle111;

use crate::ram::{
    Address,
    Word,
};

use super::ArmInstruction;

pub fn get_arm_instruction(address: Address, value: Word) -> ArmInstruction {
    let bit25_27 = (value >> 25) & 0b111;

    match bit25_27 {
        0b000 => handle000(address, value),
        0b001 => handle001(address, value),
        0b010 => handle010(address, value),
        0b011 => handle011(address, value),
        0b100 => handle100(address, value),
        0b101 => handle101(address, value),
        0b110 => handle110(address, value),
        0b111 => handle111(address, value),
        _ => unreachable!(),
    }
}
