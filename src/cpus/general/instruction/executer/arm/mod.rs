mod error;
mod helper;

pub use error::ArmExecuterError;
use helper::Helper;

use crate::cpus::{
    general::{
        instruction::{
            decode::arm::Miscellaneous,
            encodings::{
                arm::DataProcessingImmediateShift,
                encoding_fields::DataProcessingInstruction,
            },
        },
        register::{
            types::ConditionBit,
            RegisterName,
            Registers,
        },
        BitState,
        BitMaskConstants,
    },
    Architecture,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ArmExecuter<'a> {
    registers: &'a mut Registers,
    architecture: Architecture,
}

impl<'a> ArmExecuter<'a> {
    pub fn new(registers: &'a mut Registers, architecture: Architecture) -> Self {
        Self {
            registers,
            architecture,
        }
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

    pub fn miscellaneous(&mut self, misc_instruction: Miscellaneous) {
        match misc_instruction {
            Miscellaneous::MRS(data) => {
                let rd = RegisterName::from(data.rd);
                let cpsr = self.registers.get_ref_cpsr();

                if data.r_flag.is_set() {
                    let operating_mode = cpsr.get_operating_mode().unwrap();
                    let spsr = self.registers.get_spsr(operating_mode).unwrap();
                    self.registers.set_reg(rd, spsr);
                } else {
                    let cpsr_val = self.registers.get_reg(RegisterName::Cpsr);
                    self.registers.set_reg(rd, cpsr_val);
                }
            }
            Miscellaneous::MSR(data) => {
                if data.operand & BitMaskConstants::UnallocMask.as_u32(self.architecture) != 0 {
                    todo!("Unpredictable");
                }

                let byte_mask = {
                    let field_mask1 = BitState::from(data.field_mask);
                    let field_mask2 = BitState::from(data.field_mask >> 1);
                    let field_mask3 = BitState::from(data.field_mask >> 2);
                    let field_mask4 = BitState::from(data.field_mask >> 3);

                    let mut byte_mask = 0;
                    
                    if field_mask1.is_set() {
                        byte_mask |= 0x0000_00FF;
                    } 
                    if field_mask2.is_set() {
                        byte_mask |= 0x0000_FF00;
                    }
                    if field_mask3.is_set() {
                        byte_mask |= 0x00FF_0000;
                    }
                    if field_mask4.is_set() {
                        byte_mask |= 0xFF00_0000;
                    }

                    byte_mask
                };

                let cpsr = self.registers.get_ref_cpsr();
                if data.r_flag.is_unset() {
                    let mask = {
                        if cpsr.in_privileged_mode() {
                            if data.operand & BitMaskConstants::StateMask.as_u32(self.architecture) != 0 {
                                todo!("Unpredictable");
                            } else {
                                byte_mask & (
                                    BitMaskConstants::UserMask.as_u32(self.architecture)
                                    | BitMaskConstants::PrivMask.as_u32(self.architecture))
                            }
                        } else {
                            byte_mask & BitMaskConstants::UserMask.as_u32(self.architecture)
                        }
                    };

                    let cpsr_val = self.registers.get_reg(RegisterName::Cpsr);
                    let new_cpsr_val = (cpsr_val & !mask) | (data.operand & mask);
                    self.registers.set_reg(RegisterName::Cpsr, new_cpsr_val);

                } else {
                    if cpsr.current_mode_has_spsr() {
                        let mask = byte_mask & (
                            BitMaskConstants::UserMask.as_u32(self.architecture)
                            | BitMaskConstants::PrivMask.as_u32(self.architecture)
                            | BitMaskConstants::StateMask.as_u32(self.architecture)
                        );

                        let current_operating_mode = cpsr.get_operating_mode().unwrap();
                        let spsr_val = self.registers.get_spsr(current_operating_mode).unwrap();
                        let new_spsr_val = (spsr_val & !mask) | (data.operand & mask);
                        self.registers.set_spsr(new_spsr_val);
                    } else {
                        todo!("Unpredictable");
                    }
                }
            }
            Miscellaneous::BranchExchangeInstructionSetThumb(data) => {}
            Miscellaneous::BranchExchangeInstructionSetJava(data) => {}
            Miscellaneous::CountLeadingZeros(data) => {}
            Miscellaneous::BranchAndLinkExchangeInstructionSetThumb(data) => {}
            Miscellaneous::SaturatingAddSubtract(data) => {}
            Miscellaneous::SoftwareBreakpoint(data) => {}
            Miscellaneous::SignedMultipliesType2(data) => {}
            Miscellaneous::Unknown => println!("Reached unknown miscellaneous instruction, LOL"),
        }
    }

    pub fn data_processing_register_shift(&self) {}

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

    pub fn branch_and_branch_with_link(&self) {}

    pub fn coprocessor_load_and_store_and_double_register_transfers(&self) {}

    pub fn coprocessor_data_processing(&self) {}

    pub fn coprocessor_register_transfers(&self) {}

    pub fn software_interrupt(&self) {}

    pub fn unconditional_instructions(&self) {}
}

#[cfg(test)]
mod tests;
