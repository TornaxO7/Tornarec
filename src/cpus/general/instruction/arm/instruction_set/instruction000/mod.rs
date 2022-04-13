use self::miscellaneous1::{get_mrs, get_msr, branch_exchange_instruction_set_thumb, signed_multiplies_type2};

mod miscellaneous1;

pub fn handle000(address: Address, value: Word) -> ArmInstruction {
    let bit24 = (value >> 23) & 0b1;
    let bit23 = (value >> 23) & 0b1;
    let bit20 = (value >> 20) & 0b1;
    let bit7 = (value >> 7) & 0b1;
    let bit4 = (value >> 4) & 0b1;

    match (bit24, bit23, bit20, bit7, bit4) {
        (_, _, _, _, 0) => data_processing_immediate_shift(address, value),
        (1, 0, 0, _, 0) => miscellaneous_instructions1(address, value),
        (_, _, _, 0, 1) => data_processing_register_shift(address, value),
        (1, 0, 0, 0, 1) => miscellaneous_instructions2(address, value),
        (_, _, _, 1, 1) => multiplies_and_extra_load_store(address, value),
    }
}

/// ARM INSTRUCTIONS
fn data_processing_immediate_shift(address: Address, value: Word) -> ArmInstruction {
    let s = BitState::from(((value >> 20) & 0b1) != 0);
    let rn = Register::try_from((value >> 16) & 0b1111).unwrap();
    let rd = Register::try_from((value >> 12) & 0b1111).unwrap();
    let offset = AddressingMode1Offset::get_immediate(value);

    ArmInstruction {
        opcode: get_data_processing_operand(value),
        operand: ArmOperand::AddressingMode1 { s, rn, rd, offset },
        address,
        cond: ConditionCodeFlag::from(value),
    }
}

fn miscellaneous_instructions1(address: Address, value: Word) -> ArmInstruction {
    let bit21 = (value >> 21) & 0b1;
    let bit7 = (value >> 7) & 0b1;
    let bit5 = (value >> 5) & 0b1;
    let bit4 = (value >> 4) & 0b1;

    match (bit21, bit7, bit5, bit4) {
        (0, 0, 0, 0) => get_mrs(address, value),
        (1, 0, 0, 0) => get_msr(address, value),
        (1, 0, 1, 0) => branch_exchange_instruction_set_thumb(address, value),
        (1, 1, _, 0) => signed_multiplies_type2(address, value),
        (_, _, _, _) => todo!("[Unknown Misc] Figure A3-4 (page 145). Value: {}", value),
    }
}

fn data_processing_register_shift(address: Address, value: Word) -> ArmInstruction {
    let s = BitState::from(((value >> 20) & 0b1) != 0);
    let rn = Register::try_from((value >> 16) & 0b1111).unwrap();
    let rd = Register::try_from((value >> 12) & 0b1111).unwrap();
    let offset = AddressingMode1Offset::get_register_shift(value);

    ArmInstruction {
        opcode: get_data_processing_operand(value),
        operand: ArmOperand::AddressingMode1 { s, rn, rd, offset },
        address,
        cond: ConditionCodeFlag::from(value),
    }
}

fn miscellaneous_instructions2(address: Address, value: Word) -> ArmInstruction {}

fn multiplies_and_extra_load_store(address: Address, value: Word) -> ArmInstruction {}

/// HELPER FUNCTIONS
fn get_data_processing_operand(value: Word) -> ArmOpcode {
    let opcode = (value >> 21) & 0b1111;

    match opcode {
        0b0000 => ArmOpcode::AND,
        0b0001 => ArmOpcode::EOR,
        0b0010 => ArmOpcode::SUB,
        0b0011 => ArmOpcode::RSB,
        0b0100 => ArmOpcode::ADD,
        0b0101 => ArmOpcode::ADC,
        0b0110 => ArmOpcode::SBC,
        0b0111 => ArmOpcode::RSC,
        0b1000 => ArmOpcode::TST,
        0b1001 => ArmOpcode::TEQ,
        0b1010 => ArmOpcode::CMP,
        0b1011 => ArmOpcode::CMN,
        0b1100 => ArmOpcode::ORR,
        0b1101 => ArmOpcode::MOV,
        0b1110 => ArmOpcode::BIC,
        0b1111 => ArmOpcode::MVN,
        _ => unreachable!("Unknown opcode: {}", opcode),
    }
}
