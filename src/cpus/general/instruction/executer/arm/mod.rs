mod error;
mod helper;

pub use error::ArmExecuterError;
use helper::Helper;

use crate::{
    cpus::{
        general::{
            exception::ExceptionVector,
            instruction::{
                decode::arm::Miscellaneous,
                encodings::{
                    arm::DataProcessingData,
                    encoding_fields::{
                        DataProcessingInstruction,
                        SaturatingOpcode,
                    },
                },
            },
            register::{
                types::ConditionBit,
                RegisterName,
                Registers,
            },
            BitMaskConstants,
            BitState,
            Interruption,
            OperatingMode,
            OperatingState,
        },
        Architecture,
    },
    ram::data_types::DataTypeSize,
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

    pub fn data_processing(&mut self, data: DataProcessingData) {
        // some general values which are needed during this process
        let cpsr = self.registers.get_ref_cpsr();

        let rn_val = self.registers.get_reg(data.rn_reg);

        match data.opcode {
            DataProcessingInstruction::AND => {
                let rd_val = rn_val & data.shifter_operand.val;
                self.registers.set_reg(data.rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if data.rd_reg == RegisterName::R15 {
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
                self.registers.set_reg(data.rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if data.rd_reg == RegisterName::R15 {
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
                self.registers.set_reg(data.rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if data.rd_reg == RegisterName::R15 {
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
                self.registers.set_reg(data.rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if data.rd_reg == RegisterName::R15 {
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
                self.registers.set_reg(data.rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if data.rd_reg == RegisterName::R15 {
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
                self.registers.set_reg(data.rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if data.rd_reg == RegisterName::R15 {
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
                self.registers.set_reg(data.rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if data.rd_reg == RegisterName::R15 {
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
                self.registers.set_reg(data.rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if data.rd_reg == RegisterName::R15 {
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

                self.registers.set_reg(data.rd_reg, rd_val);

                if data.s_flag.is_set() {
                    if data.rd_reg == RegisterName::R15 {
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
                let cpsr = self.registers.get_ref_cpsr();

                if data.r_flag.is_set() {
                    let operating_mode = cpsr.get_operating_mode().unwrap();
                    let spsr = self.registers.get_spsr(operating_mode).unwrap();
                    self.registers.set_reg(data.rd_reg, spsr);
                } else {
                    let cpsr_val = self.registers.get_reg(RegisterName::Cpsr);
                    self.registers.set_reg(data.rd_reg, cpsr_val);
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
                            if data.operand & BitMaskConstants::StateMask.as_u32(self.architecture)
                                != 0
                            {
                                todo!("Unpredictable");
                            } else {
                                byte_mask
                                    & (BitMaskConstants::UserMask.as_u32(self.architecture)
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
                        let mask = byte_mask
                            & (BitMaskConstants::UserMask.as_u32(self.architecture)
                                | BitMaskConstants::PrivMask.as_u32(self.architecture)
                                | BitMaskConstants::StateMask.as_u32(self.architecture));

                        let current_operating_mode = cpsr.get_operating_mode().unwrap();
                        let spsr_val = self.registers.get_spsr(current_operating_mode).unwrap();
                        let new_spsr_val = (spsr_val & !mask) | (data.operand & mask);
                        self.registers.set_spsr(new_spsr_val);
                    } else {
                        todo!("Unpredictable");
                    }
                }
            }
            Miscellaneous::BX(data) => {
                // set the pc value
                let rm_reg = RegisterName::from(data.rm_reg);
                let rm_val = self.registers.get_reg(rm_reg);

                self.registers
                    .set_reg(RegisterName::Pc, rm_val & 0xFFFF_FFFE);

                // Set T bit if needed
                let cpsr = self.registers.get_mut_cpsr();
                let new_operatin_state = {
                    let bit_value = BitState::from(rm_val);
                    OperatingState::from(bit_value)
                };

                cpsr.set_operating_state(new_operatin_state);
            }
            // The CPUs of the NintendoDS doesn't support Jazelle
            Miscellaneous::BXJ(_) => unreachable!("{}", ArmExecuterError::NoJazelleSupport),
            Miscellaneous::CLZ(data) => {
                if self.architecture == Architecture::ARMv5TE {
                    let rm_reg = RegisterName::from(data.rm_reg);
                    let rd_reg = RegisterName::from(data.rd_reg);

                    let rm_val = self.registers.get_reg(rm_reg);

                    self.registers.set_reg(rd_reg, rm_val.leading_zeros());
                }
            }
            Miscellaneous::BLX(data) => {
                if self.architecture == Architecture::ARMv5TE {
                    self.registers.move_pc_to_lr();

                    // Adjust the cpsr
                    {
                        let cpsr = self.registers.get_mut_cpsr();
                        cpsr.set_operating_state(OperatingState::Thumb);
                    }

                    // update the pc
                    let pc_val = self.registers.get_reg(RegisterName::Pc);
                    self.registers.set_reg(
                        RegisterName::Pc,
                        pc_val
                            + (Helper::sign_extend(data.signed_immed_24, 24) << 2)
                            + (data.h_flag.get_as_u32() << 1),
                    );
                } else {
                    unreachable!("{}", ArmExecuterError::ARMv4TExecutesARM5vTE);
                }
            }
            Miscellaneous::QADDOrQSUB(data) => {
                // NOTE: Use of R15 as RD, RM or RN isn't covered as unpredictable
                if self.architecture == Architecture::ARMv5TE {
                    let rm_val = self.registers.get_reg(data.rm_reg);
                    let rn_val = self.registers.get_reg(data.rn_reg);

                    match data.opcode {
                        SaturatingOpcode::QADD => {
                            let (sum, _) = rm_val.overflowing_add(rn_val);
                            let val = Helper::signed_sat(sum as i32, 32);
                            self.registers.set_reg(data.rd_reg, val as u32);

                            if Helper::signed_does_sat(sum as i32, 32) {
                                let cpsr = self.registers.get_mut_cpsr();
                                cpsr.set_condition_bit(ConditionBit::Q, BitState::Set);
                            }
                        }
                        SaturatingOpcode::QSUB => {
                            let (subtraction, _) = rm_val.overflowing_sub(rn_val);
                            let val = Helper::signed_sat(subtraction as i32, 32);
                            self.registers.set_reg(data.rd_reg, val as u32);

                            if Helper::signed_does_sat(subtraction as i32, 32) {
                                let cpsr = self.registers.get_mut_cpsr();
                                cpsr.set_condition_bit(ConditionBit::Q, BitState::Set);
                            }
                        }
                    }
                } else {
                    unreachable!("{}", ArmExecuterError::ARMv4TExecutesARM5vTE);
                }
            }
            // DEBUGGER: _data variable
            Miscellaneous::BKPT(_data) => {
                let bkpt_address = self.registers.get_adjusted_pc();
                self.registers.set_reg(
                    RegisterName::R14Abt,
                    (bkpt_address + DataTypeSize::Custom(4)).get_as_u32(),
                );

                let cpsr_val = self.registers.get_reg(RegisterName::Cpsr);
                self.registers.set_reg(RegisterName::SpsrAbt, cpsr_val);

                {
                    let cpsr = self.registers.get_mut_cpsr();
                    cpsr.set_operating_mode(OperatingMode::Abt);
                    cpsr.set_operating_state(OperatingState::Arm);
                    cpsr.set_interrupt_bit(Interruption::Irq, BitState::Set);
                }

                self.registers
                    .set_reg(RegisterName::Pc, ExceptionVector::PABT);
            }
            Miscellaneous::SignedMultipliesType2(data) => {
                if self.architecture == Architecture::ARMv5TE {
                    let rm_val = self.registers.get_reg(data.rm_reg);
                    let rs_val = self.registers.get_reg(data.rs_reg);
                    let rn_val = self.registers.get_reg(data.rn_reg);

                    let operand1 = if data.x.is_set() {
                        Helper::sign_extend(rm_val & 0b1111_1111_1111_1111, 15)
                    } else {
                        Helper::sign_extend(rm_val >> 16, 15)
                    };

                    let operand2 = if data.y.is_set() {
                        Helper::sign_extend(rs_val & 0b1111_1111_1111_1111, 15)
                    } else {
                        Helper::sign_extend(rs_val >> 16, 15)
                    };

                    let (calculation, overflowed) = {
                        let (multiplication, overflowed1) = operand1.overflowing_mul(operand2);
                        let (result, overflowed2) = multiplication.overflowing_add(rn_val);
                        (result, overflowed1 | overflowed2)
                    };
                    self.registers.set_reg(data.rd_reg, calculation);

                    if overflowed {
                        let cpsr = self.registers.get_mut_cpsr();
                        cpsr.set_condition_bit(ConditionBit::Q, BitState::Set);
                    }
                }
            }
            Miscellaneous::Unknown => println!("Reached unknown miscellaneous instruction, LOL"),
        }
    }

    pub fn multiplies(&self) {}

    pub fn extra_load_and_stores(&self) {}

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
