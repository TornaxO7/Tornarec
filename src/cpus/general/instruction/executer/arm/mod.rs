mod error;

pub use error::ArmExecuterError;

use crate::{
    cpus::general::{
        instruction::encodings::{
            arm::{
                DataProcessingImmediateShift,
                BranchAndBranchWithLink,
            },
            encoding_fields::DataProcessingInstruction,
        },
        register::{
            types::ConditionBit,
            NormalizedRegister,
            RegisterName,
            Registers,
        },
        BitState,
    },
    ram::Ram,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ArmExecuter<'a> {
    registers: &'a mut Registers,
    ram: &'a Ram,
}

impl<'a> ArmExecuter<'a> {
    pub fn new(registers: &'a mut Registers, ram: &'a Ram) -> Self {
        Self { registers, ram }
    }

    pub fn data_processing_immediate_shift(&mut self, data: DataProcessingImmediateShift) {
        
        let cpsr = self.registers.get_ref_cpsr();

        match data.opcode {
            DataProcessingInstruction::AND => {
                let rd = data.rn & data.shifter_operand.shifter_operand;

                if data.s_flag.is_set() {
                    if NormalizedRegister::from(rd) == NormalizedRegister::from(RegisterName::R15) {
                        if let Err(err) = self.registers.move_current_spsr_to_cpsr() {
                            panic!("{}", err);
                        }
                    } else {
                        let cpsr = self.registers.get_mut_cpsr();

                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(rd >> 31));
                        cpsr.set_condition_bit(ConditionBit::Z, BitState::from(rd == 0));
                        cpsr.set_condition_bit(ConditionBit::C, data.shifter_operand.shifter_carry_out);
                    }
                }
            }
            DataProcessingInstruction::EOR => {}
            DataProcessingInstruction::SUB => {}
            DataProcessingInstruction::RSB => {}
            DataProcessingInstruction::ADD => {
                let rd = data.rn + data.shifter_operand.shifter_operand;

                if data.s_flag.is_set() {
                    if NormalizedRegister::from(rd) == NormalizedRegister::from(RegisterName::R15) {
                        if let Err(err) = self.registers.move_current_spsr_to_cpsr() {
                            panic!("{}", err);
                        }
                    } else {
                        let carry = data.rn + data.shifter_operand.shifter_operand;
                        let overflow: Option<u32> = data.rn
                            .checked_add(data.shifter_operand.shifter_operand);

                        let cpsr = self.registers.get_mut_cpsr();
                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(rd >> 31));
                        cpsr.set_condition_bit(ConditionBit::Z, BitState::from(rd == 0));
                        cpsr.set_condition_bit(ConditionBit::C, BitState::from(carry >> 31));
                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(overflow.is_none()));
                    }
                }
            }
            DataProcessingInstruction::ADC => {
                let c_flag = cpsr.get_condition_bit(ConditionBit::C);
                let rd = data.rn + data.shifter_operand.shifter_operand + c_flag.get_as_u32();

                if data.s_flag.is_set() {
                    if NormalizedRegister::from(rd) == NormalizedRegister::from(RegisterName::R15) {
                        if let Err(err) = self.registers.move_current_spsr_to_cpsr() {
                            panic!("{}", err);
                        }
                    } else {
                        let carry = data.rn + data.shifter_operand.shifter_operand + c_flag.get_as_u32();
                        let overflow: Option<u32> = data.rn
                            .checked_add(data.shifter_operand.shifter_operand)
                            .and_then(|rn| rn.checked_add(c_flag.get_as_u32()));

                        let cpsr = self.registers.get_mut_cpsr();
                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(rd >> 31));
                        cpsr.set_condition_bit(ConditionBit::Z, BitState::from(rd == 0));
                        cpsr.set_condition_bit(ConditionBit::C, BitState::from(carry >> 31));
                        cpsr.set_condition_bit(ConditionBit::V, BitState::from(overflow.is_none()));
                    }
                }
            },
            DataProcessingInstruction::SBC => {}
            DataProcessingInstruction::RSC => {}
            DataProcessingInstruction::TST => {}
            DataProcessingInstruction::TEQ => {}
            DataProcessingInstruction::CMP => {}
            DataProcessingInstruction::CMN => {}
            DataProcessingInstruction::ORR => {}
            DataProcessingInstruction::MOV => {}
            DataProcessingInstruction::BIC => {}
            DataProcessingInstruction::MVN => {}
        }
    }

    pub fn miscellaneous_1(&self) {}

    pub fn data_processing_register_shift(&self) {}

    pub fn miscellaneous2(&self) {}

    pub fn multiplies(&self) {}

    pub fn extra_load_and_stores(&self) {}

    pub fn data_processing_immediate(&self) {}

    pub fn undefined_instruction(&self) {}

    pub fn move_immediate_to_status_register(&self) {}

    pub fn load_and_store_immediate_offset(&self) {}

    pub fn load_and_store_register_offset(&self) {}

    pub fn media_instructions(&self) {}

    pub fn architecturally_undefined(&self) {}

    pub fn load_and_store_multiple(&self) {}

    pub fn branch_and_branch_with_link(&self, _data: BranchAndBranchWithLink) {
        // let pc = self.registers.get_reg(RegisterName::Pc);
        //
        // if data.l_flag.is_set() {
        //     self.registers.set_reg(RegisterName::Lr, pc - 32);
        // }
    }

    pub fn coprocessor_load_and_store_and_double_register_transfers(&self) {}

    pub fn coprocessor_data_processing(&self) {}

    pub fn coprocessor_register_transfers(&self) {}

    pub fn software_interrupt(&self) {}

    pub fn unconditional_instructions(&self) {}
}
