use crate::{
    cpus::general::{
        condition_code_flag::ConditionCodeFlag,
        instruction::arm::{
            encoding_fields::MSRType,
            opcode::ArmOpcode,
            operand::ArmOperand,
            ArmInstruction,
            BitState,
            Register,
        },
    },
    ram::{
        Address,
        Word,
    },
};

use std::convert::TryFrom;

pub fn get_mrs(address: Address, value: Word) -> ArmInstruction {
    let cond = ConditionCodeFlag::from(value);
    let operand = ArmOperand::MRS {
        r: BitState::from(((value >> 22) & 0b1) != 0),
        rd: Register::try_from((value >> 12) & 0b1111).unwrap(),
    };

    let sbo = (value >> 16) & 0b1111;
    let sbz = value & 0b1111_1111_1111;

    if sbo != 0b1111 {
        todo!("[SBO] Figure A3-4 (page 224)");
    } else if sbz != 0 {
        todo!("[SBZ] Figure A3-4 (page 224)");
    }

    ArmInstruction {
        opcode: ArmOpcode::MRS,
        operand,
        cond,
        address,
    }
}

// Move register to CPSR
pub fn get_msr(address: Address, value: Word) -> ArmInstruction {
    let bit25 = BitState::from(((value >> 25) & 0b1) != 0);
    let sbo = (value >> 12) & 0b1111;

    let r = BitState::from(((value >> 22) & 0b1) != 0);
    let field_mask = u8::try_from((value >> 16) & 0b1111).unwrap();
    let cond = ConditionCodeFlag::from(value);

    if sbo != 0b1111 {
        todo!("[SBO] A4.1.39 (page 226)");
    }

    if bit25 {
        let operand = ArmOperand::MSR {
            r,
            field_mask,
            msr_type: MSRType::get_immediate(value),
        };

        ArmInstruction {
            opcode: ArmOpcode::MSR,
            operand,
            cond,
            address,
        }
    } else {
        let sbz = (value >> 8) & 0b1111;

        if sbz != 0 {
            todo!("[SBZ] A4.1.39 (page 226)");
        }

        let operand = ArmOperand::MSR {
            r,
            field_mask,
            msr_type: MSRType::get_register(value),
        };

        ArmInstruction {
            opcode: ArmOpcode::MSR,
            operand,
            cond,
            address,
        }
    }
}

pub fn get_bx(address: Address, value: Word) -> ArmInstruction {
    let rm = Register::try_from(value & 0b1111).unwrap();
    let sbo = (value >> 8) & 0b1111_1111_1111;

    if sbo != 0b1111_1111_1111 {
        todo!("[SBO] A4.1.10 (page 170)");
    }

    ArmInstruction {
        opcode: ArmOpcode::BX,
        operand: ArmOperand::BX(rm),
        cond: ConditionCodeFlag::from(value),
        address,
    }
}

pub fn get_clz(address: Address, value: Word) -> ArmInstruction {
}

pub fn get_blx(address: Address, value: Word) -> ArmInstruction {
}

pub fn get_saturating_add_subtract(address: Address, value: Word) -> ArmInstruction {
}

pub fn get_bkpt(address: Address, value: Word) -> ArmInstruction {
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
    let rn = Register::try_from((value >> 12) & 0b1111).unwrap();
    let sbz = rn;
    let operand = ArmOperand::HalfwordMultiply {
        rd: Register::try_from((value >> 16) & 0b1111).unwrap(),
        rn,
        rs: Register::try_from((value >> 8) & 0b1111).unwrap(),
        y: BitState::from(((value >> 6) & 0b1) != 0),
        x: BitState::from(((value >> 5) & 0b1) != 0),
        rm: Register::try_from(value & 0b1111).unwrap(),
    };

    match op {
        // SMUL<x><y>
        0b11 => {
            if sbz != 0 {
                todo!("[SBZ] A4.1.86 (page 316)");
            }

            ArmInstruction {
                opcode: ArmOpcode::SMULXY,
                operand,
                cond,
                address,
            }
        }
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

    let cond = ConditionCodeFlag::from(value);
    let rn = Register::try_from((value >> 12) & 0b1111).unwrap();
    let sbz = rn;
    let operand = ArmOperand::WordHalfwordMultiply {
        rd: Register::try_from((value >> 16) & 0b1111).unwrap(),
        rn,
        rs: Register::try_from((value >> 8) & 0b1111).unwrap(),
        y: BitState::from(((value >> 6) & 0b1) != 0),
        rm: Register::try_from(value & 0b1111).unwrap(),
    };

    // SMULW<y>
    if bit5 {
        if sbz != 0 {
            todo!("[SBZ] A4.1.88 (page 320)");
        }

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

#[cfg(test)]
mod tests {
    use crate::{
        cpus::general::{
            condition_code_flag::ConditionCodeFlag,
            instruction::arm::{
                encoding_fields::MSRType,
                opcode::ArmOpcode,
                operand::ArmOperand,
                ArmInstruction,
                BitState,
                Register,
            },
        },
        ram::Address,
    };

    use super::{
        get_bx,
        get_mrs,
        get_msr,
        get_signed_multiplies_type2,
    };

    const ADDRESS: Address = Address::from(0 as u32);

    #[test]
    fn test_get_mrs() {
        let value = 0b0000_0001_0100_1111_1111_0000_0000_0000;

        assert_eq!(
            ArmInstruction {
                opcode: ArmOpcode::MRS,
                operand: ArmOperand::MRS {
                    r: BitState::from(true),
                    rd: Register::from(0b1111),
                },
                cond: ConditionCodeFlag::EQ,
                address: ADDRESS,
            },
            get_mrs(ADDRESS, value)
        );
    }

    #[test]
    #[should_panic]
    fn test_get_mrs_sbo() {
        let value = 0b0000_0001_0100_0000_1111_0000_0000_0000;
        get_mrs(ADDRESS, value);
    }

    #[test]
    #[should_panic]
    fn test_get_mrs_sbz() {
        let value = 0b0000_0001_0100_1111_1111_1111_1111_1111;
        get_mrs(ADDRESS, value);
    }

    #[test]
    fn test_get_msr_immediate() {
        let value = 0b0000_0011_0110_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmInstruction {
                opcode: ArmOpcode::MSR,
                operand: ArmOperand::MSR {
                    r: BitState::from(true),
                    field_mask: 0b1111,
                    msr_type: MSRType::Immediate {
                        rotate_imm: 0b1111,
                        immediate: 0b1111_1111,
                    },
                },
                cond: ConditionCodeFlag::EQ,
                address: ADDRESS,
            },
            get_msr(ADDRESS, value)
        );
    }

    #[test]
    fn test_get_msr_register() {
        let value = 0b0000_0001_0110_1111_1111_0000_0000_1111;

        assert_eq!(
            ArmInstruction {
                opcode: ArmOpcode::MSR,
                operand: ArmOperand::MSR {
                    r: BitState::from(true),
                    field_mask: 0b1111,
                    msr_type: MSRType::Register(Register::from(0b1111)),
                },
                cond: ConditionCodeFlag::EQ,
                address: ADDRESS,
            },
            get_msr(ADDRESS, value)
        );
    }

    #[test]
    #[should_panic]
    fn test_get_msr_immediate_sbo() {
        let value = 0b0000_0011_0110_1111_0000_1111_1111_1111;
        get_msr(ADDRESS, value);
    }

    #[test]
    #[should_panic]
    fn test_get_msr_register_sbo() {
        let value = 0b0000_0001_0110_1111_0000_0000_0000_1111;
        get_msr(ADDRESS, value);
    }

    #[test]
    #[should_panic]
    fn test_get_msr_register_sbz() {
        let value = 0b0000_0001_0110_1111_1111_1111_0000_1111;
        get_msr(ADDRESS, value);
    }

    #[test]
    fn test_get_bx() {
        let value = 0b0000_0001_0010_1111_1111_1111_0001_1111;

        assert_eq!(
            ArmInstruction {
                opcode: ArmOpcode::BX,
                operand: ArmOperand::BX(Register::from(0b1111)),
                cond: ConditionCodeFlag::EQ,
                address: ADDRESS,
            },
            get_bx(ADDRESS, value)
        );
    }

    #[test]
    #[should_panic]
    fn test_get_bx_sbo() {
        let value = 0b0000_0001_0010_0000_0000_0000_0001_1111;
        get_bx(ADDRESS, value);
    }

    #[test]
    fn test_smul_xy() {
        let value = 0b0000_0001_0110_1111_0000_1111_1110_1111;

        assert_eq!(
            ArmInstruction {
                opcode: ArmOpcode::SMULXY,
                operand: ArmOperand::HalfwordMultiply {
                    rd: Register::from(0b1111),
                    rn: Register::from(0b0000),
                    rs: Register::from(0b1111),
                    y: BitState::from(true),
                    x: BitState::from(true),
                    rm: Register::from(0b1111),
                },
                cond: ConditionCodeFlag::EQ,
                address: ADDRESS,
            },
            get_signed_multiplies_type2(ADDRESS, value)
        );
    }

    #[test]
    #[should_panic]
    fn test_smul_xy_sbz() {
        let value = 0b0000_0001_0110_1111_1111_1111_1110_1111;
        get_signed_multiplies_type2(ADDRESS, value);
    }

    #[test]
    fn test_smla_xy() {
        let value = 0b0000_0001_0000_1111_1111_1111_1110_1111;

        assert_eq!(
            ArmInstruction {
                opcode: ArmOpcode::SMLAXY,
                operand: ArmOperand::HalfwordMultiply {
                    rd: Register::from(0b1111),
                    rn: Register::from(0b1111),
                    rs: Register::from(0b1111),
                    y: BitState::from(true),
                    x: BitState::from(true),
                    rm: Register::from(0b1111),
                },
                cond: ConditionCodeFlag::EQ,
                address: ADDRESS,
            },
            get_signed_multiplies_type2(ADDRESS, value)
        );
    }

    #[test]
    fn test_smlal_xy() {
        let value = 0b0000_0001_0100_1111_1111_1111_1110_1111;

        assert_eq!(
            ArmInstruction {
                opcode: ArmOpcode::SMLALXY,
                operand: ArmOperand::HalfwordMultiply {
                    rd: Register::from(0b1111),
                    rn: Register::from(0b1111),
                    rs: Register::from(0b1111),
                    y: BitState::from(true),
                    x: BitState::from(true),
                    rm: Register::from(0b1111),
                },
                cond: ConditionCodeFlag::EQ,
                address: ADDRESS,
            },
            get_signed_multiplies_type2(ADDRESS, value)
        );
    }

    #[test]
    fn test_smalw_y() {
        let value = 0b0000_0001_0010_1111_0000_1111_1110_1111;

        assert_eq!(
            ArmInstruction {
                opcode: ArmOpcode::SMULWY,
                operand: ArmOperand::WordHalfwordMultiply {
                    rd: Register::from(0b1111),
                    rn: Register::from(0b0000),
                    rs: Register::from(0b1111),
                    y: BitState::from(true),
                    rm: Register::from(0b1111),
                },
                cond: ConditionCodeFlag::EQ,
                address: ADDRESS,
            },
            get_signed_multiplies_type2(ADDRESS, value)
        );
    }

    #[test]
    #[should_panic]
    fn test_smalw_y_sbz() {
        let value = 0b0000_0001_0010_1111_1111_1111_1110_1111;
        get_signed_multiplies_type2(ADDRESS, value);
    }

    #[test]
    fn test_smlaw_y() {
        let value = 0b0000_0001_0010_1111_1111_1111_1100_1111;

        assert_eq!(
            ArmInstruction {
                opcode: ArmOpcode::SMLAWY,
                operand: ArmOperand::WordHalfwordMultiply {
                    rd: Register::from(0b1111),
                    rn: Register::from(0b1111),
                    rs: Register::from(0b1111),
                    y: BitState::from(true),
                    rm: Register::from(0b1111),
                },
                cond: ConditionCodeFlag::EQ,
                address: ADDRESS,
            },
            get_signed_multiplies_type2(ADDRESS, value)
        );
    }
}
