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

pub fn get_mrs(address: Address, value: Word) -> ArmInstruction {
    ArmInstruction {
        opcode: ArmOpcode::MRS,
        operand: ArmOperand::get_mrs(value),
        cond: ConditionCodeFlag::from(value),
        address,
    }
}

// Move register to CPSR
pub fn get_msr(address: Address, value: Word) -> ArmInstruction {
    ArmInstruction {
        opcode: ArmOpcode::MSR,
        operand: ArmOperand::get_msr(value),
        cond: ConditionCodeFlag::from(value),
        address,
    }
}

pub fn get_bx(address: Address, value: Word) -> ArmInstruction {
    ArmInstruction {
        opcode: ArmOpcode::BX,
        operand: ArmOperand::get_bx(value),
        cond: ConditionCodeFlag::from(value),
        address,
    }
}

pub fn get_clz(address: Address, value: Word) -> ArmInstruction {
    ArmInstruction {
        opcode: ArmOpcode::CLZ,
        operand: ArmOperand::get_clz(value),
        cond: ConditionCodeFlag::from(value),
        address,
    }
}

pub fn get_blx(address: Address, value: Word) -> ArmInstruction {
    ArmInstruction {
        opcode: ArmOpcode::BLX,
        operand: ArmOperand::get_blx(value),
        cond: ConditionCodeFlag::from(value),
        address,
    }
}

pub fn get_saturating_add_subtract(address: Address, value: Word) -> ArmInstruction {
    let cond = ConditionCodeFlag::from(value);
    let operand = ArmOperand::get_saturating_add_subtract(value);

    let bit22_21 = (value >> 21) & 0b11;

    match bit22_21 {
        // QADD
        0b00 => ArmInstruction {
            opcode: ArmOpcode::QADD,
            operand,
            cond,
            address,
        },
        // QSUB
        0b01 => ArmInstruction {
            opcode: ArmOpcode::QSUB,
            operand,
            cond,
            address,
        },
        // QDADD
        0b10 => ArmInstruction {
            opcode: ArmOpcode::QDADD,
            operand,
            cond,
            address,
        },
        // QDSUB
        0b11 => ArmInstruction {
            opcode: ArmOpcode::QDSUB,
            operand,
            cond,
            address,
        },
        _ => unreachable!(
            "[Unknown Opcode] Unknown Saturating add and subtract opcode: {:#032b}",
            value
        ),
    }
}

pub fn get_bkpt(address: Address, value: Word) -> ArmInstruction {
    ArmInstruction {
        opcode: ArmOpcode::BKPT,
        operand: ArmOperand::get_bkpt(value),
        cond: ConditionCodeFlag::from(value),
        address,
    }
}

// Differ bettwen halfwor multiply and word+halfword multiply
pub fn get_signed_multiplies_type2(address: Address, value: Word) -> ArmInstruction {
    let op = (value >> 21) & 0b11;

    if op == 0b01 {
        word_x_halfword_multiply(address, value)
    } else {
        halfword_multiply(address, value)
    }
}

fn halfword_multiply(address: Address, value: Word) -> ArmInstruction {
    let op = (value >> 21) & 0b11;
    let cond = ConditionCodeFlag::from(value);
    let operand = ArmOperand::get_halfword_multiply(value);

    match op {
        // SMUL<x><y>
        0b11 => ArmInstruction {
            opcode: ArmOpcode::SMULXY,
            operand,
            cond,
            address,
        },
        // SMLA<x><y>
        0b00 => ArmInstruction {
            opcode: ArmOpcode::SMLAXY,
            operand,
            cond,
            address,
        },
        // SMLAL<x><y>
        0b10 => ArmInstruction {
            opcode: ArmOpcode::SMLALXY,
            operand,
            cond,
            address,
        },
        _ => todo!("Unknown instruction; A3.5.3 (page 119)"),
    }
}

fn word_x_halfword_multiply(address: Address, value: Word) -> ArmInstruction {
    let bit5 = BitState::from(((value >> 5) & 0b1) != 0);

    let operand = ArmOperand::get_word_halfword_multiply(value);
    let cond = ConditionCodeFlag::from(value);

    // SMULW<y>
    if bit5 {
        ArmInstruction {
            opcode: ArmOpcode::SMULWY,
            operand,
            cond,
            address,
        }
    }
    // SMLAW<y>
    else {
        ArmInstruction {
            opcode: ArmOpcode::SMLAWY,
            operand,
            cond,
            address,
        }
    }
}
