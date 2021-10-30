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
