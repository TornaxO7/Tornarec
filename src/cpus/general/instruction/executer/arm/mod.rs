mod error;
mod helper;

pub use error::ArmExecuterError;
use helper::Helper;

use crate::{
    cpus::general::{
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
                let rd_val = rn_val - data.shifter_operand.val;
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
                        cpsr.set_condition_bit(
                            ConditionBit::V,
                            Helper::overflow_from(vec![
                                rn_val as i32,
                                -(data.shifter_operand.val as i32),
                            ]),
                        );
                    }
                }
            }
            DataProcessingInstruction::RSB => {
                let rd_val = data.shifter_operand.val - rn_val;
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
                        cpsr.set_condition_bit(
                            ConditionBit::V,
                            Helper::overflow_from(vec![
                                data.shifter_operand.val as i32,
                                rn_val as i32,
                            ]),
                        );
                    }
                }
            }
            DataProcessingInstruction::ADD => {
                let rd_val = rn_val + data.shifter_operand.val;
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
                        cpsr.set_condition_bit(
                            ConditionBit::N,
                            Helper::overflow_from(vec![
                                rn_val as i32,
                                data.shifter_operand.val as i32,
                            ]),
                        );
                    }
                }
            }
            DataProcessingInstruction::ADC => {
                let c_flag = cpsr.get_condition_bit(ConditionBit::C);

                let rd_val = rn_val + data.shifter_operand.val + c_flag.get_as_u32();
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
                        cpsr.set_condition_bit(
                            ConditionBit::V,
                            Helper::overflow_from(vec![
                                rn_val as i32,
                                data.shifter_operand.val as i32,
                                c_flag.get_as_i32(),
                            ]),
                        );
                    }
                }
            }
            DataProcessingInstruction::SBC => {
                let c_flag = cpsr.get_condition_bit(ConditionBit::C);
                let rd_val = rn_val - data.shifter_operand.val - (!c_flag).get_as_u32();
                self.registers.set_reg(rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if RegisterName::from(rd_val) == RegisterName::R15 {
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
                        cpsr.set_condition_bit(
                            ConditionBit::N,
                            Helper::overflow_from(vec![
                                rn_val as i32,
                                data.shifter_operand.val as i32,
                                (!c_flag).get_as_i32(),
                            ]),
                        );
                    }
                }
            }
            DataProcessingInstruction::RSC => {
                let c_flag = cpsr.get_condition_bit(ConditionBit::C);
                let rd_val = data.shifter_operand.val - rn_val - (!c_flag).get_as_u32();
                self.registers.set_reg(rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if RegisterName::from(rd_val) == RegisterName::R15 {
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
                        cpsr.set_condition_bit(
                            ConditionBit::N,
                            Helper::overflow_from(vec![
                                data.shifter_operand.val as i32,
                                -(rn_val as i32),
                                -(!c_flag).get_as_i32(),
                            ]),
                        );
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
                let alu_out = rn_val - data.shifter_operand.val;

                let cpsr = self.registers.get_mut_cpsr();
                cpsr.set_condition_bit(ConditionBit::N, BitState::from(alu_out >> 31));
                cpsr.set_condition_bit(ConditionBit::Z, BitState::from(alu_out == 0));
                cpsr.set_condition_bit(
                    ConditionBit::C,
                    !Helper::borrow_from(vec![rn_val, data.shifter_operand.val]),
                );
                cpsr.set_condition_bit(
                    ConditionBit::V,
                    !Helper::overflow_from(vec![rn_val as i32, -(data.shifter_operand.val as i32)]),
                );
            }
            DataProcessingInstruction::CMN => {
                let alu_out = rn_val + data.shifter_operand.val;

                let cpsr = self.registers.get_mut_cpsr();
                cpsr.set_condition_bit(ConditionBit::N, BitState::from(alu_out >> 31));
                cpsr.set_condition_bit(ConditionBit::Z, BitState::from(alu_out == 0));
                cpsr.set_condition_bit(
                    ConditionBit::C,
                    !Helper::carry_from(vec![rn_val, data.shifter_operand.val]),
                );
                cpsr.set_condition_bit(
                    ConditionBit::V,
                    !Helper::overflow_from(vec![rn_val as i32, data.shifter_operand.val as i32]),
                );
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
