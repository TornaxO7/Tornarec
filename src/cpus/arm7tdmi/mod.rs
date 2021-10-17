pub mod error;

use crate::cpus::general::{
    bit_state::BitState,
    pipeline::Pipeline,
    operating_state::OperatingState,
    operating_mode::OperatingMode,
    exception::{Exception, ExceptionStack},
    register::{Registers, RegisterName, Cpsr},
    interruption::Interruption,
};

use crate::ram::{
    Ram,
    Address,
    data_types::DataTypeSize
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Arm7TDMI {
    registers: Registers,

    pipeline: Pipeline,
    exception_stack: ExceptionStack,
}

impl Arm7TDMI {

    pub fn reset(&self) -> Self {
        Self::default()
    }

    pub fn step(&mut self, ram: &Ram) {
        self.fetch(ram);
        self.decode();
        self.execute();
    }

    pub fn fetch(&mut self, ram: &Ram) {
        let pc = self.registers.get_reg(RegisterName::Pc);
        let cpsr = self.registers.get_ref_cpsr();

        let start = Address::from(pc);

        match cpsr.get_operating_state() {
            OperatingState::Arm => self.pipeline.fetch(ram, start, DataTypeSize::Word),
            OperatingState::Thumb => self.pipeline.fetch(ram, start, DataTypeSize::Halfword),
        };
    }

    pub fn decode(&mut self) {
        let cpsr = self.registers.get_ref_cpsr();
        self.pipeline.decode(cpsr);
    }

    pub fn execute(&mut self) {
        // let decoded_instruction = self.pipeline.get_decoded_instruction();

        // let arm_executer = ArmExecuter::new();
        // let thumb_executer = ThumbExecuter::new();
        //
        // match decoded_instruction {
        //     InstructionMap::Arm(arm_instruction) => match arm_instruction {
        //         ArmInstruction::DataProcessingImmediateShift(data) => ,
        //         ArmInstruction::Miscellaneous1(data) => ,
        //         ArmInstruction::DataProcessingRegisterShift(data) => ,
        //         ArmInstruction::Miscellaneous2(data) => ,
        //         ArmInstruction::Multiplies(data) => ,
        //         ArmInstruction::ExtraLoadAndStores(data) => ,
        //         ArmInstruction::DataProcessingImmediate(data) => ,
        //         ArmInstruction::UndefinedInstruction => ,
        //         ArmInstruction::MoveImmediateToStatusRegister(data) => ,
        //         ArmInstruction::LoadAndStoreImmediateOffset(data) => ,
        //         ArmInstruction::LoadAndStoreRegisterOffset(data) => ,
        //         ArmInstruction::MediaInstructions => ,
        //         ArmInstruction::ArchitecturallyUndefined,
        //         ArmInstruction::LoadAndStoreMultiple(data) => ,
        //         ArmInstruction::BranchAndBranchWithLink(data) => ,
        //         ArmInstruction::CoprocessorLoadAndStoreAndDoubleRegisterTransfers(data) => ,
        //         ArmInstruction::CoprocessorDataProcessing(data) => ,
        //         ArmInstruction::CoprocessorRegisterTransfers(data) => ,
        //         ArmInstruction::SoftwareInterrupt => ,
        //     },
        //     InstructionMap::Thumb(thumb_instruction) => match thumb_instruction {
        //         ThumbInstruction::ShiftByImmediate(data) => ,
        //         ThumbInstruction::AddSubtractRegister(data) => ,
        //         ThumbInstruction::AddSubtractImmediate(data) => ,
        //         ThumbInstruction::AddSubtractCompareMoveImmediate(data) => ,
        //         ThumbInstruction::DataProcessingRegister(data) => ,
        //         ThumbInstruction::SpecialDataProcessing(data) => ,
        //         ThumbInstruction::UnconditionalBranch(data) => ,
        //         ThumbInstruction::BranchExchangeInstructionSet(data) => ,
        //         ThumbInstruction::LoadFromLiteralPool(data) => ,
        //         ThumbInstruction::LoadStoreRegisterOffset(data) => ,
        //         ThumbInstruction::LoadStoreWordByteImmediateOffset(data) => ,
        //         ThumbInstruction::LoadStoreHalfwordImmediateOffset(data) => ,
        //         ThumbInstruction::LoadStoretoFromStack(data) => ,
        //         ThumbInstruction::AddToSpOrPc(data) => ,
        //         ThumbInstruction::LoadStoreMultiple(data) => ,
        //         ThumbInstruction::ConditionalBranch(data) => ,
        //         ThumbInstruction::UndefinedInstruction,
        //         ThumbInstruction::SoftwareInterrupt(data) => ,
        //         ThumbInstruction::BlxSuffix(data) => ,
        //         ThumbInstruction::BlOrBlxPrefix(data) => ,
        //         ThumbInstruction::BlSuffix(data) => ,
        //
        //         ThumbInstruction::// miscellaneous instructions,
        //         ThumbInstruction::AdjustStackPointer(data) => ,
        //         ThumbInstruction::PushPopRegisterList(data) => ,
        //         ThumbInstruction::SoftwareBreakpoint(data) => ,
        //
        //     },
        //     InstructionMap::Noop => (),
        // }
    }

    pub fn enter_exception(&mut self, exception: Exception) {

        if self.exception_stack.push(exception.clone()).is_some() {

            let pc_val = self.registers.get_reg(RegisterName::Pc);
            let old_cpsr = self.registers.get_ref_cpsr().clone();

            let in_arm_state = old_cpsr.get_operating_state() == OperatingState::Arm;

            match exception {
                // Exception::Bl => {
                //     if in_arm_state {
                //         self.lr[OperatingMode::as_usize(OperatingMode::Sys)] = GeneralRegister::from(self.pc.clone() + 2);
                //     } else {
                //         self.lr[OperatingMode::as_usize(OperatingMode::Sys)] = GeneralRegister::from(self.pc.clone() + 4);
                //     }
                //     self.cpsr.set_operating_mode(OperatingMode::Sys);
                //
                // },
                Exception::Swi => {
                    if in_arm_state {
                        self.registers.set_reg(RegisterName::LrSvc, pc_val + 2);
                    } else {
                        self.registers.set_reg(RegisterName::LrSvc, pc_val + 4);
                    }

                    self.registers.set_reg(RegisterName::SpsrSvc, old_cpsr.get_as_u32());
                    self.registers.set_reg(RegisterName::Pc, Exception::Swi.get_exception_vector().get_as_u32());

                    let cpsr: &mut Cpsr = self.registers.get_mut_cpsr();
                    cpsr.set_operating_mode(OperatingMode::Svc);
                },
                Exception::Udef => {
                    if in_arm_state {
                        self.registers.set_reg(RegisterName::LrUnd, pc_val + 2);
                    } else {
                        self.registers.set_reg(RegisterName::LrUnd, pc_val + 4);
                    }

                    self.registers.set_reg(RegisterName::SpsrUnd, old_cpsr.get_as_u32());
                    self.registers.set_reg(RegisterName::Pc, Exception::Udef.get_exception_vector().get_as_u32());

                    let cpsr: &mut Cpsr = self.registers.get_mut_cpsr();
                    cpsr.set_operating_mode(OperatingMode::Und);
                },
                Exception::Pabt => {
                    self.registers.set_reg(RegisterName::LrAbt, pc_val + 4);
                    self.registers.set_reg(RegisterName::SpsrAbt, old_cpsr.get_as_u32());
                    self.registers.set_reg(RegisterName::Pc, Exception::Pabt.get_exception_vector().get_as_u32());

                    let cpsr: &mut Cpsr = self.registers.get_mut_cpsr();
                    cpsr.set_operating_mode(OperatingMode::Abt);
                },
                Exception::Fiq  => {
                    self.registers.set_reg(RegisterName::LrFiq, pc_val + 4);
                    self.registers.set_reg(RegisterName::SpsrFiq, old_cpsr.get_as_u32());
                    self.registers.set_reg(RegisterName::Pc, Exception::Fiq.get_exception_vector().get_as_u32());
                    
                    let cpsr: &mut Cpsr = self.registers.get_mut_cpsr();
                    cpsr.set_operating_mode(OperatingMode::Fiq);
                    cpsr.set_interrupt_bit(Interruption::Fiq, BitState::Set);
                },
                Exception::Irq  => {
                    self.registers.set_reg(RegisterName::LrIrq, pc_val + 4);
                    self.registers.set_reg(RegisterName::SpsrIrq, old_cpsr.get_as_u32());
                    self.registers.set_reg(RegisterName::Pc, Exception::Fiq.get_exception_vector().get_as_u32());

                    let cpsr: &mut Cpsr = self.registers.get_mut_cpsr();
                    cpsr.set_operating_mode(OperatingMode::Irq);
                },
                Exception::Dabt => {
                    self.registers.set_reg(RegisterName::LrAbt, pc_val + 8);
                    self.registers.set_reg(RegisterName::SpsrAbt, old_cpsr.get_as_u32());
                    self.registers.set_reg(RegisterName::Pc, Exception::Dabt.get_exception_vector().get_as_u32());

                    let cpsr: &mut Cpsr = self.registers.get_mut_cpsr();
                    cpsr.set_operating_mode(OperatingMode::Abt);
                },
                Exception::Reset => {
                    self.registers.set_reg(RegisterName::Pc, Exception::Reset.get_exception_vector().get_as_u32());

                    let cpsr: &mut Cpsr = self.registers.get_mut_cpsr();
                    cpsr.set_operating_mode(OperatingMode::Svc);
                    cpsr.set_interrupt_bit(Interruption::Fiq, BitState::Set);
                },
            };

            let cpsr: &mut Cpsr = self.registers.get_mut_cpsr();
            cpsr.set_interrupt_bit(Interruption::Irq, BitState::Set);
            cpsr.set_operating_state(OperatingState::Arm);
        }
    }
}
