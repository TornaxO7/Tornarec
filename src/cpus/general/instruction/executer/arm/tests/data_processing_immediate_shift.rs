use crate::cpus::general::{
    instruction::{
        encodings::{
            arm::DataProcessingImmediateShift,
            encoding_fields::{
                DataProcessingInstruction,
                ShifterOperand,
            },
        },
        executer::ArmExecuter,
    },
    register::{
        types::{
            ConditionBit,
            ConditionBits,
        },
        RegisterName,
        Registers,
    },
    BitState,
};

#[test]
fn and() {
    let mut registers = Registers::default();
    registers.set_reg(RegisterName::R1, 0b1001);
    let mut arm_executer = ArmExecuter::new(&mut registers);

    let data = DataProcessingImmediateShift {
        opcode: DataProcessingInstruction::AND,
        s_flag: BitState::Set,
        rn: 0b1,
        rd: 0b10,
        shifter_operand: ShifterOperand {
            val: 0b1111,
            shifter_carry_out: BitState::Set,
        },
    };

    arm_executer.data_processing_immediate_shift(data);

    let mut expected_registers = Registers::default();
    expected_registers.set_reg(RegisterName::R1, 0b1001);
    expected_registers.set_reg(RegisterName::R2, 0b1001);
    {
        let cpsr = expected_registers.get_mut_cpsr();
        cpsr.set_condition_bits(ConditionBits {
            n: BitState::Unset,
            z: BitState::Unset,
            c: BitState::Set,
            v: BitState::Unset,
        });
    }

    assert_eq!(
        expected_registers, registers,
        "{:#?}, {:#?}",
        &expected_registers, &registers
    );
}

#[test]
fn eor() {
    let mut registers = Registers::default();
    registers.set_reg(RegisterName::R2, 0b0111);
    let mut arm_executer = ArmExecuter::new(&mut registers);

    let data = DataProcessingImmediateShift {
        opcode: DataProcessingInstruction::EOR,
        s_flag: BitState::Set,
        rn: 0b0010,
        rd: 0b0100,
        shifter_operand: ShifterOperand {
            val: 0b1100,
            shifter_carry_out: BitState::Unset,
        },
    };

    arm_executer.data_processing_immediate_shift(data);

    let mut expected_registers = Registers::default();
    expected_registers.set_reg(RegisterName::R2, 0b0111);
    expected_registers.set_reg(RegisterName::R4, 0b1011);

    assert_eq!(
        expected_registers, registers,
        "{:#?} {:#?}",
        &expected_registers, &registers
    );
}

#[test]
fn sub() {
    let mut registers = Registers::default();
    registers.set_reg(RegisterName::R8, 42);

    let mut arm_executer = ArmExecuter::new(&mut registers);

    let data = DataProcessingImmediateShift {
        opcode: DataProcessingInstruction::SUB,
        s_flag: BitState::Set,
        rn: 0b1000,
        rd: 0b1,
        shifter_operand: ShifterOperand {
            val: 12,
            shifter_carry_out: BitState::Set,
        },
    };

    arm_executer.data_processing_immediate_shift(data);

    let mut expected_registers = Registers::default();
    expected_registers.set_reg(RegisterName::R8, 42);
    expected_registers.set_reg(RegisterName::R1, 30);
    {
        let cpsr = expected_registers.get_mut_cpsr();
        cpsr.set_condition_bit(ConditionBit::C, BitState::Set);
    }

    assert_eq!(
        expected_registers, registers,
        "{:#?} {:#?}",
        &expected_registers, &registers
    );
}

#[test]
fn rsb() {
    let mut registers = Registers::default();
    registers.set_reg(RegisterName::R5, 12);

    let mut arm_executer = ArmExecuter::new(&mut registers);

    let data = DataProcessingImmediateShift {
        opcode: DataProcessingInstruction::RSB,
        s_flag: BitState::Set,
        rn: 0b0101,
        rd: 0b1,
        shifter_operand: ShifterOperand {
            val: 2,
            shifter_carry_out: BitState::Set,
        },
    };

    arm_executer.data_processing_immediate_shift(data);

    let mut expected_registers = Registers::default();
    expected_registers.set_reg(RegisterName::R5, 12);
    expected_registers.set_reg(RegisterName::R1, u32::MAX - 9);
    {
        let cpsr = expected_registers.get_mut_cpsr();
        cpsr.set_condition_bit(ConditionBit::N, BitState::Set);
        cpsr.set_condition_bit(ConditionBit::V, BitState::Set);
    }

    assert_eq!(
        expected_registers, registers,
        "{:#?} {:#?}",
        &expected_registers, &registers
    );
}

#[test]
fn add() {
    let mut registers = Registers::default();
    registers.set_reg(RegisterName::R7, 10);

    let mut arm_executer = ArmExecuter::new(&mut registers);

    let data = DataProcessingImmediateShift {
        opcode: DataProcessingInstruction::ADD,
        s_flag: BitState::Set,
        rn: 0b0111,
        rd: 0b11,
        shifter_operand: ShifterOperand {
            val: 32,
            shifter_carry_out: BitState::Set,
        },
    };

    arm_executer.data_processing_immediate_shift(data);

    let mut expected_registers = Registers::default();
    expected_registers.set_reg(RegisterName::R7, 10);
    expected_registers.set_reg(RegisterName::R3, 42);

    assert_eq!(
        expected_registers, registers,
        "{:#?} {:#?}",
        &expected_registers, &registers
    );
}

#[test]
fn adc() {
    let mut registers = Registers::default();
    registers.set_reg(RegisterName::R3, 0xFFFF_FF01);
    {
        let cpsr = registers.get_mut_cpsr();
        cpsr.set_condition_bit(ConditionBit::C, BitState::Set);
    }
    let mut arm_executer = ArmExecuter::new(&mut registers);

    let data = DataProcessingImmediateShift {
        opcode: DataProcessingInstruction::ADC,
        s_flag: BitState::Set,
        rn: 0b0011,
        rd: 0b1011,
        shifter_operand: ShifterOperand {
            val: 0xFF,
            shifter_carry_out: BitState::Unset,
        },
    };

    arm_executer.data_processing_immediate_shift(data);

    let mut expected_registers = Registers::default();
    expected_registers.set_reg(RegisterName::R3, 0xFFFF_FF01);
    expected_registers.set_reg(RegisterName::R11, 1);
    {
        let cpsr = expected_registers.get_mut_cpsr();
        cpsr.set_condition_bits(ConditionBits {
            n: BitState::Unset,
            z: BitState::Unset,
            c: BitState::Set,
            v: BitState::Set,
        });
    }

    assert_eq!(
        expected_registers, registers,
        "{:#?} {:#?}",
        &expected_registers, &registers
    );
}

#[test]
fn sbc() {
    let mut registers = Registers::default();
    registers.set_reg(RegisterName::R1, 10);

    let mut arm_executer = ArmExecuter::new(&mut registers);

    let data = DataProcessingImmediateShift {
        opcode: DataProcessingInstruction::SBC,
        s_flag: BitState::Set,
        rn: 0b1,
        rd: 0b10,
        shifter_operand: ShifterOperand {
            val: 10,
            shifter_carry_out: BitState::Set,
        },
    };

    arm_executer.data_processing_immediate_shift(data);

    let mut expected_registers = Registers::default();
    expected_registers.set_reg(RegisterName::R1, 10);
    expected_registers.set_reg(RegisterName::R2, u32::MAX);
    {
        let cpsr = expected_registers.get_mut_cpsr();
        cpsr.set_condition_bit(ConditionBit::N, BitState::Set);
        cpsr.set_condition_bit(ConditionBit::V, BitState::Set);
    }

    assert_eq!(
        expected_registers, registers,
        "{:#?} {:#?}",
        &expected_registers, &registers
    );
}

#[test]
fn rsc() {
    let mut registers = Registers::default();
    registers.set_reg(RegisterName::R3, 18);

    let mut arm_executer = ArmExecuter::new(&mut registers);

    let data = DataProcessingImmediateShift {
        opcode: DataProcessingInstruction::RSC,
        s_flag: BitState::Set,
        rn: 0b11,
        rd: 0b1,
        shifter_operand: ShifterOperand {
            val: 8,
            shifter_carry_out: BitState::Set,
        },
    };

    arm_executer.data_processing_immediate_shift(data);

    let mut expected_registers = Registers::default();
    expected_registers.set_reg(RegisterName::R3, 18);
    expected_registers.set_reg(RegisterName::R1, u32::MAX - 10);
    {
        let cpsr = expected_registers.get_mut_cpsr();
        cpsr.set_condition_bit(ConditionBit::N, BitState::Set);
        cpsr.set_condition_bit(ConditionBit::V, BitState::Set);
    }

    assert_eq!(
        expected_registers, registers,
        "{:#?} {:#?}",
        &expected_registers, &registers
    );
}

#[test]
fn tst() {
    let mut registers = Registers::default();
    registers.set_reg(RegisterName::R1, 0b1111_1111_1111_1111__1111_1111_1111_1111);

    let mut arm_executer = ArmExecuter::new(&mut registers);

    let data = DataProcessingImmediateShift {
        opcode: DataProcessingInstruction::TST,
        s_flag: BitState::Set,
        rn: 0b1,
        rd: 0b0,
        shifter_operand: ShifterOperand {
            val: 1 << 31,
            shifter_carry_out: BitState::Set,
        },
    };

    arm_executer.data_processing_immediate_shift(data);

    let mut expected_registers = Registers::default();
    expected_registers.set_reg(RegisterName::R1, u32::MAX);
    {
        let cpsr = expected_registers.get_mut_cpsr();
        cpsr.set_condition_bit(ConditionBit::N, BitState::Set);
        cpsr.set_condition_bit(ConditionBit::C, BitState::Set);
    }

    assert_eq!(
        expected_registers, registers,
        "{:#?} {:#?}",
        &expected_registers, &registers
    );
}

#[test]
fn teq() {
    let mut registers = Registers::default();
    registers.set_reg(RegisterName::R1, 0b1111_1111_1111_1111__1111_1111_1111_1111);

    let mut arm_executer = ArmExecuter::new(&mut registers);

    let data = DataProcessingImmediateShift {
        opcode: DataProcessingInstruction::TEQ,
        s_flag: BitState::Set,
        rn: 0b1,
        rd: 0b0,
        shifter_operand: ShifterOperand {
            val: (1 << 31) - 1,
            shifter_carry_out: BitState::Set,
        },
    };

    arm_executer.data_processing_immediate_shift(data);

    let mut expected_registers = Registers::default();
    expected_registers.set_reg(RegisterName::R1, u32::MAX);
    {
        let cpsr = expected_registers.get_mut_cpsr();
        cpsr.set_condition_bit(ConditionBit::N, BitState::Set);
        cpsr.set_condition_bit(ConditionBit::C, BitState::Set);
    }

    assert_eq!(
        expected_registers, registers,
        "{:#?} {:#?}",
        &expected_registers, &registers
    );
}

#[test]
fn cmp() {
    let mut registers = Registers::default();
    registers.set_reg(RegisterName::R1, 1);

    let mut arm_executer = ArmExecuter::new(&mut registers);

    let data = DataProcessingImmediateShift {
        opcode: DataProcessingInstruction::CMP,
        s_flag: BitState::Set,
        rn: 0b1,
        rd: 0b0,
        shifter_operand: ShifterOperand {
            val: 11,
            shifter_carry_out: BitState::Set,
        },
    };

    arm_executer.data_processing_immediate_shift(data);

    let mut expected_registers = Registers::default();
    expected_registers.set_reg(RegisterName::R1, 1);
    {
        let cpsr = expected_registers.get_mut_cpsr();
        cpsr.set_condition_bit(ConditionBit::N, BitState::Set);
        cpsr.set_condition_bit(ConditionBit::V, BitState::Set);
    }

    assert_eq!(
        expected_registers, registers,
        "{:#?} {:#?}",
        &expected_registers, &registers
    );
}
