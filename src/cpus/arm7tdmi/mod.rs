pub mod error;

// use error::Arm7TDMIError;

use crate::cpus::general::{
    bit_state::BitState,
    pipeline::Pipeline,
    operating_state::OperatingState,
    operating_mode::OperatingMode,
    exception::{Exception, ExceptionStack},
    register::{Spsr, FiqReg, Cpsr, GeneralRegister},
    interruption::Interruption,
    InstructionMap,
    instruction::{
        state::{ArmInstruction, ThumbInstruction},
        executer::{ArmExecuter, ThumbExecuter},
    },
};

use crate::ram::{
    Ram,
    Address,
    data_types::DataTypeSize
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Arm7TDMI {
    r0_7:   [GeneralRegister; 8],
    r8_r12: [FiqReg; 4],
    // r13
    sp:     [GeneralRegister; OperatingMode::AMOUNT_MODES],
    // r14
    lr:     [GeneralRegister; OperatingMode::AMOUNT_MODES],
    // r15
    pc:     GeneralRegister,
    cpsr:   Cpsr,
    spsr:   Spsr,

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
        let start = Address::from(self.pc.clone());

        match self.cpsr.get_operating_state() {
            OperatingState::Arm => self.pipeline.fetch(ram, start, DataTypeSize::Word),
            OperatingState::Thumb => self.pipeline.fetch(ram, start, DataTypeSize::Halfword),
        };
    }

    pub fn decode(&mut self) {
        self.pipeline.decode(&self.cpsr);
    }

    pub fn execute(&mut self) {
        let decoded_instruction = self.pipeline.get_decoded_instruction();

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

            let in_arm_state = self.cpsr.get_operating_state() == OperatingState::Arm;

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
                        self.lr[OperatingMode::as_usize(OperatingMode::Svc)] = GeneralRegister::from(self.pc.clone() + 2);
                    } else {
                        self.lr[OperatingMode::as_usize(OperatingMode::Svc)] = GeneralRegister::from(self.pc.clone() + 4);
                    }

                    self.spsr.svc = self.cpsr.clone();
                    self.cpsr.set_operating_mode(OperatingMode::Svc);
                    self.pc.set_value(
                        Exception::Swi.get_exception_vector()
                            .get_as_u32()
                    );
                },
                Exception::Udef => {
                    if in_arm_state {
                        self.lr[OperatingMode::as_usize(OperatingMode::Und)] = GeneralRegister::from(self.pc.clone() + 2);
                    } else {
                        self.lr[OperatingMode::as_usize(OperatingMode::Und)] = GeneralRegister::from(self.pc.clone() + 4);
                    }

                    self.spsr.und = self.cpsr.clone();
                    self.cpsr.set_operating_mode(OperatingMode::Und);
                    self.pc.set_value(
                        Exception::Udef.get_exception_vector()
                            .get_as_u32()
                    );
                },
                Exception::Pabt => {
                    self.lr[OperatingMode::as_usize(OperatingMode::Abt)] = GeneralRegister::from(self.pc.clone() + 4);
                    self.spsr.abt = self.cpsr.clone();
                    self.cpsr.set_operating_mode(OperatingMode::Abt);
                    self.pc.set_value(
                        Exception::Pabt.get_exception_vector()
                            .get_as_u32()
                    );
                },
                Exception::Fiq  => {
                    self.lr[OperatingMode::as_usize(OperatingMode::Fiq)] = GeneralRegister::from(self.pc.clone() + 4);
                    self.spsr.fiq = self.cpsr.clone();
                    self.cpsr.set_operating_mode(OperatingMode::Fiq);
                    self.cpsr.set_interrupt_bit(Interruption::Fiq, BitState::Set);
                    self.pc.set_value(
                        Exception::Fiq.get_exception_vector()
                            .get_as_u32()
                    );
                },
                Exception::Irq  => {
                    self.lr[OperatingMode::as_usize(OperatingMode::Irq)] = GeneralRegister::from(self.pc.clone() + 4);
                    self.spsr.irq = self.cpsr.clone();
                    self.cpsr.set_operating_mode(OperatingMode::Irq);
                    self.pc.set_value(
                        Exception::Fiq.get_exception_vector()
                            .get_as_u32()
                    );
                },
                Exception::Dabt => {
                    self.lr[OperatingMode::as_usize(OperatingMode::Abt)] = GeneralRegister::from(self.pc.clone() + 8);
                    self.spsr.abt = self.cpsr.clone();
                    self.cpsr.set_operating_mode(OperatingMode::Abt);
                    self.pc.set_value(
                        Exception::Dabt.get_exception_vector()
                            .get_as_u32()
                    );
                },
                Exception::Reset => {
                    self.cpsr.set_operating_mode(OperatingMode::Svc);
                    self.cpsr.set_interrupt_bit(Interruption::Fiq, BitState::Set);
                    self.pc.set_value(
                        Exception::Reset.get_exception_vector()
                            .get_as_u32()
                    );
                },
            };

            self.cpsr.set_interrupt_bit(Interruption::Irq, BitState::Set);
            self.cpsr.set_operating_state(OperatingState::Arm);
        }
    }
}
