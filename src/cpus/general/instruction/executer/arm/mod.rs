mod error;
mod helper;

pub use error::ArmExecuterError;
use helper::Helper;

use crate::cpus::general::{
    instruction::encodings::{
        arm::{
            BranchAndBranchWithLink,
            DataProcessingImmediateShift,
        },
        encoding_fields::DataProcessingInstruction,
    },
    register::{
        types::ConditionBit,
        RegisterName,
        Registers,
    },
    BitState,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ArmExecuter<'a> {
    registers: &'a mut Registers,
}

impl<'a> ArmExecuter<'a> {
    pub fn new(registers: &'a mut Registers) -> Self {
        Self { registers }
    }

    pub fn data_processing_immediate_shift(&mut self, data: DataProcessingImmediateShift) {
        // some general values which are needed during this process
        let cpsr = self.registers.get_ref_cpsr();
        let rd_reg = RegisterName::from(data.rd);

        let rn_reg = RegisterName::from(data.rn);
        let rn_val = self.registers.get_reg(rn_reg);

        match data.opcode {
            DataProcessingInstruction::AND => {
                let rd_val = rn_val & data.shifter_operand.val;
                self.registers.set_reg(rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if rd_reg == RegisterName::R15 {
                        self.registers.move_current_spsr_to_cpsr();
                    } else {
                        let cpsr = self.registers.get_mut_cpsr();

                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(rd_val >> 31));
                        cpsr.set_condition_bit(ConditionBit::Z, BitState::from(rd_val == 0));
                        cpsr.set_condition_bit(
                            ConditionBit::C,
                            data.shifter_operand.shifter_carry_out,
                        );
                    }
                }
            }
            DataProcessingInstruction::EOR => {
                let rd_val = rn_val ^ data.shifter_operand.val;
                self.registers.set_reg(rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if rd_reg == RegisterName::R15 {
                        self.registers.move_current_spsr_to_cpsr();
                    } else {
                        let cpsr = self.registers.get_mut_cpsr();

                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(rd_val >> 31));
                        cpsr.set_condition_bit(ConditionBit::Z, BitState::from(rd_val == 0));
                        cpsr.set_condition_bit(
                            ConditionBit::C,
                            data.shifter_operand.shifter_carry_out,
                        );
                    }
                }
            }
            DataProcessingInstruction::SUB => {
                let (rd_val, overflowed) = rn_val.overflowing_sub(data.shifter_operand.val);
                self.registers.set_reg(rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if rd_reg == RegisterName::R15 {
                        self.registers.move_current_spsr_to_cpsr();
                    } else {
                        let cpsr = self.registers.get_mut_cpsr();

                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(rd_val >> 31));
                        cpsr.set_condition_bit(ConditionBit::Z, BitState::from(rd_val == 0));
                        cpsr.set_condition_bit(
                            ConditionBit::C,
                            !Helper::borrow_from(vec![rn_val, data.shifter_operand.val]),
                        );
                        cpsr.set_condition_bit(ConditionBit::V, BitState::from(overflowed));
                    }
                }
            }
            DataProcessingInstruction::RSB => {
                let (rd_val, overflowed) = data.shifter_operand.val.overflowing_sub(rn_val);
                self.registers.set_reg(rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if rd_reg == RegisterName::R15 {
                        self.registers.move_current_spsr_to_cpsr();
                    } else {
                        let cpsr = self.registers.get_mut_cpsr();

                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(rd_val >> 31));
                        cpsr.set_condition_bit(ConditionBit::Z, BitState::from(rd_val == 0));
                        cpsr.set_condition_bit(
                            ConditionBit::C,
                            !Helper::borrow_from(vec![data.shifter_operand.val, rn_val]),
                        );
                        cpsr.set_condition_bit(ConditionBit::V, BitState::from(overflowed));
                    }
                }
            }
            DataProcessingInstruction::ADD => {
                let (rd_val, overflowed) = rn_val.overflowing_add(data.shifter_operand.val);
                self.registers.set_reg(rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if rd_reg == RegisterName::R15 {
                        self.registers.move_current_spsr_to_cpsr();
                    } else {
                        let cpsr = self.registers.get_mut_cpsr();
                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(rd_val >> 31));
                        cpsr.set_condition_bit(ConditionBit::Z, BitState::from(rd_val == 0));
                        cpsr.set_condition_bit(
                            ConditionBit::C,
                            Helper::carry_from(vec![rn_val, data.shifter_operand.val]),
                        );
                        cpsr.set_condition_bit(ConditionBit::V, BitState::from(overflowed));
                    }
                }
            }
            DataProcessingInstruction::ADC => {
                let c_flag = cpsr.get_condition_bit(ConditionBit::C);

                let (rd_val, overflowed) = {
                    let (rd_val, overflowed1) = rn_val.overflowing_add(data.shifter_operand.val);
                    let (rd_val, overflowed2) = rd_val.overflowing_add(c_flag.get_as_u32());

                    (rd_val, overflowed1 || overflowed2)
                };
                self.registers.set_reg(rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if rd_reg == RegisterName::R15 {
                        self.registers.move_current_spsr_to_cpsr();
                    } else {
                        let cpsr = self.registers.get_mut_cpsr();
                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(rd_val >> 31));
                        cpsr.set_condition_bit(ConditionBit::Z, BitState::from(rd_val == 0));
                        cpsr.set_condition_bit(
                            ConditionBit::C,
                            Helper::carry_from(vec![
                                rn_val,
                                data.shifter_operand.val,
                                c_flag.get_as_u32(),
                            ]),
                        );
                        cpsr.set_condition_bit(ConditionBit::V, BitState::from(overflowed));
                    }
                }
            }
            DataProcessingInstruction::SBC => {
                let c_flag = cpsr.get_condition_bit(ConditionBit::C);
                let (rd_val, overflowed) = {
                    let (rd_val, overflowed1) = rn_val.overflowing_sub(data.shifter_operand.val);
                    let (rd_val, overflowed2) = rd_val.overflowing_sub((!c_flag).get_as_u32());
                    (rd_val, overflowed1 || overflowed2)
                };
                self.registers.set_reg(rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if rd_reg == RegisterName::R15 {
                        self.registers.move_current_spsr_to_cpsr();
                    } else {
                        let cpsr = self.registers.get_mut_cpsr();

                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(rd_val >> 31));
                        cpsr.set_condition_bit(ConditionBit::Z, BitState::from(rd_val == 0));
                        cpsr.set_condition_bit(
                            ConditionBit::C,
                            !Helper::borrow_from(vec![
                                rn_val,
                                data.shifter_operand.val,
                                (!c_flag).get_as_u32(),
                            ]),
                        );
                        cpsr.set_condition_bit(ConditionBit::V, BitState::from(overflowed));
                    }
                }
            }
            DataProcessingInstruction::RSC => {
                let c_flag = cpsr.get_condition_bit(ConditionBit::C);
                let (rd_val, overflowed) = {
                    let (rd_val, overflowed1) = data.shifter_operand.val.overflowing_sub(rn_val);
                    let (rd_val, overflowed2) = rd_val.overflowing_sub((!c_flag).get_as_u32());
                    (rd_val, overflowed1 || overflowed2)
                };
                self.registers.set_reg(rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if rd_reg == RegisterName::R15 {
                        self.registers.move_current_spsr_to_cpsr();
                    } else {
                        let cpsr = self.registers.get_mut_cpsr();

                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(rd_val >> 31));
                        cpsr.set_condition_bit(ConditionBit::Z, BitState::from(rd_val == 0));
                        cpsr.set_condition_bit(
                            ConditionBit::C,
                            !Helper::borrow_from(vec![
                                data.shifter_operand.val,
                                rn_val,
                                (!c_flag).get_as_u32(),
                            ]),
                        );
                        cpsr.set_condition_bit(ConditionBit::V, BitState::from(overflowed));
                    }
                }
            }
            DataProcessingInstruction::TST | DataProcessingInstruction::TEQ => {
                let alu_out = match data.opcode {
                    DataProcessingInstruction::TST => rn_val & data.shifter_operand.val,
                    DataProcessingInstruction::TEQ => rn_val ^ data.shifter_operand.val,
                    _ => unreachable!("That shouldn't happen..."),
                };

                let cpsr = self.registers.get_mut_cpsr();
                cpsr.set_condition_bit(ConditionBit::N, BitState::from(alu_out >> 31));
                cpsr.set_condition_bit(ConditionBit::Z, BitState::from(alu_out == 0));
                cpsr.set_condition_bit(ConditionBit::C, data.shifter_operand.shifter_carry_out);
            }
            DataProcessingInstruction::CMP => {
                let (alu_out, overflowed) = rn_val.overflowing_sub(data.shifter_operand.val);

                let cpsr = self.registers.get_mut_cpsr();
                cpsr.set_condition_bit(ConditionBit::N, BitState::from(alu_out >> 31));
                cpsr.set_condition_bit(ConditionBit::Z, BitState::from(alu_out == 0));
                cpsr.set_condition_bit(
                    ConditionBit::C,
                    !Helper::borrow_from(vec![rn_val, data.shifter_operand.val]),
                );
                cpsr.set_condition_bit(ConditionBit::V, BitState::from(overflowed));
            }
            DataProcessingInstruction::CMN => {
                let (alu_out, overflowed) = rn_val.overflowing_add(data.shifter_operand.val);

                let cpsr = self.registers.get_mut_cpsr();
                cpsr.set_condition_bit(ConditionBit::N, BitState::from(alu_out >> 31));
                cpsr.set_condition_bit(ConditionBit::Z, BitState::from(alu_out == 0));
                cpsr.set_condition_bit(
                    ConditionBit::C,
                    Helper::carry_from(vec![rn_val, data.shifter_operand.val]),
                );
                cpsr.set_condition_bit(ConditionBit::V, BitState::from(overflowed));
            }
            DataProcessingInstruction::ORR
            | DataProcessingInstruction::MOV
            | DataProcessingInstruction::BIC
            | DataProcessingInstruction::MVN => {
                let rd_val = match data.opcode {
                    DataProcessingInstruction::ORR => rn_val | data.shifter_operand.val,
                    DataProcessingInstruction::MOV => data.shifter_operand.val,
                    DataProcessingInstruction::BIC => rn_val & (!data.shifter_operand.val),
                    DataProcessingInstruction::MVN => !data.shifter_operand.val,
                    _ => unreachable!(),
                };

                self.registers.set_reg(rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if rd_reg == RegisterName::R15 {
                        self.registers.move_current_spsr_to_cpsr();
                    } else {
                        let cpsr = self.registers.get_mut_cpsr();
                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(rd_val >> 31));
                        cpsr.set_condition_bit(ConditionBit::Z, BitState::from(rd_val == 0));
                        cpsr.set_condition_bit(
                            ConditionBit::C,
                            data.shifter_operand.shifter_carry_out,
                        );
                    }
                }
            }
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

    pub fn branch_and_branch_with_link(&self, _data: BranchAndBranchWithLink) {}

    pub fn coprocessor_load_and_store_and_double_register_transfers(&self) {}

    pub fn coprocessor_data_processing(&self) {}

    pub fn coprocessor_register_transfers(&self) {}

    pub fn software_interrupt(&self) {}

    pub fn unconditional_instructions(&self) {}
}

#[cfg(test)]
mod tests;
