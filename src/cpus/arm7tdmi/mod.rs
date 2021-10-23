pub mod error;

use crate::{
    cpus::general::{
        bit_state::BitState,
        exception::{
            Exception,
            ExceptionStack,
            ExceptionVector,
        },
        instruction::executer::{
            ArmExecuter,
            ThumbExecuter,
        },
        interruption::Interruption,
        operating_mode::OperatingMode,
        operating_state::OperatingState,
        pipeline::Pipeline,
        register::{
            Cpsr,
            RegisterName,
            Registers,
        },
    },
    ram::{
        data_types::DataTypeSize,
        Ram,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Arm7TDMI {
    pub registers: Registers,

    pipeline: Pipeline,
    exception_stack: ExceptionStack,
}

impl Arm7TDMI {
    pub fn reset(&self) -> Self {
        Self::default()
    }

    pub fn step(&mut self, ram: &mut Ram) {
        self.fetch(ram);
        self.decode();
        self.execute(ram);
    }

    pub fn fetch(&mut self, ram: &Ram) {
        let cpsr = self.registers.get_ref_cpsr();
        let pc = self.registers.get_pc();

        match cpsr.get_operating_state() {
            OperatingState::Arm => self.pipeline.fetch(ram, pc, DataTypeSize::Word),
            OperatingState::Thumb => self.pipeline.fetch(ram, pc, DataTypeSize::Halfword),
        };
    }

    pub fn decode(&mut self) {
        self.pipeline.decode(&self.registers);
    }

    pub fn execute(&mut self, ram: &mut Ram) {
        let _decoded_instruction = self.pipeline.get_decoded_instruction();
        
        let _arm = ArmExecuter::new(&mut self.registers, ram);
        let _thumb = ThumbExecuter::new();

        // match decoded_instruction {
        //     InstructionMap::Arm(arm_instruction) => match arm_instruction {
        //         ArmDecoded::DataProcessingImmediateShift(data) =>
        //             arm.data_processing_immediate_shift(data),
        //         ArmDecoded::Miscellaneous1(data) => ,
        //         ArmDecoded::DataProcessingRegisterShift(data) => ,
        //         ArmDecoded::Miscellaneous2(data) => ,
        //         ArmDecoded::Multiplies(data) => ,
        //         ArmDecoded::ExtraLoadAndStores(data) => ,
        //         ArmDecoded::DataProcessingImmediate(data) => ,
        //         ArmDecoded::UndefinedInstruction => ,
        //         ArmDecoded::MoveImmediateToStatusRegister(data) => ,
        //         ArmDecoded::LoadAndStoreImmediateOffset(data) => ,
        //         ArmDecoded::LoadAndStoreRegisterOffset(data) => ,
        //         ArmDecoded::MediaInstructions => ,
        //         ArmDecoded::ArchitecturallyUndefined,
        //         ArmDecoded::LoadAndStoreMultiple(data) => ,
        //         ArmDecoded::BranchAndBranchWithLink(data) => ,
        //         ArmDecoded::
        // CoprocessorLoadAndStoreAndDoubleRegisterTransfers(data) => ,
        //         ArmDecoded::CoprocessorDataProcessing(data) => ,
        //         ArmDecoded::CoprocessorRegisterTransfers(data) => ,
        //         ArmDecoded::SoftwareInterrupt => ,
        //     },
        //     InstructionMap::Thumb(thumb_instruction) => match
        // thumb_instruction {         ThumbInstruction::
        // ShiftByImmediate(data) => ,         ThumbInstruction::
        // AddSubtractRegister(data) => ,         ThumbInstruction::
        // AddSubtractImmediate(data) => ,         ThumbInstruction::
        // AddSubtractCompareMoveImmediate(data) => ,
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
            let cpsr = self.registers.get_ref_cpsr().clone();

            let in_arm_state = cpsr.get_operating_state() == OperatingState::Arm;

            match exception {
                Exception::Swi => {
                    if in_arm_state {
                        self.registers.set_reg(RegisterName::LrSvc, pc_val + 2);
                    } else {
                        self.registers.set_reg(RegisterName::LrSvc, pc_val + 4);
                    }

                    self.registers
                        .set_reg(RegisterName::SpsrSvc, cpsr.get_as_u32());
                    self.registers
                        .set_reg(RegisterName::Pc, ExceptionVector::SWI);
                }
                Exception::Udef => {
                    if in_arm_state {
                        self.registers.set_reg(RegisterName::LrUnd, pc_val + 2);
                    } else {
                        self.registers.set_reg(RegisterName::LrUnd, pc_val + 4);
                    }

                    self.registers
                        .set_reg(RegisterName::SpsrUnd, cpsr.get_as_u32());
                    self.registers
                        .set_reg(RegisterName::Pc, ExceptionVector::UDEF);
                }
                Exception::Pabt => {
                    self.registers.set_reg(RegisterName::LrAbt, pc_val + 4);
                    self.registers
                        .set_reg(RegisterName::SpsrAbt, cpsr.get_as_u32());
                    self.registers
                        .set_reg(RegisterName::Pc, ExceptionVector::PABT);
                }
                Exception::Fiq => {
                    self.registers.set_reg(RegisterName::LrFiq, pc_val + 4);
                    self.registers
                        .set_reg(RegisterName::SpsrFiq, cpsr.get_as_u32());
                    self.registers
                        .set_reg(RegisterName::Pc, ExceptionVector::FIQ);
                }
                Exception::Irq => {
                    self.registers.set_reg(RegisterName::LrIrq, pc_val + 4);
                    self.registers
                        .set_reg(RegisterName::SpsrIrq, cpsr.get_as_u32());
                    self.registers
                        .set_reg(RegisterName::Pc, ExceptionVector::FIQ);
                }
                Exception::Dabt => {
                    self.registers.set_reg(RegisterName::LrAbt, pc_val + 8);
                    self.registers
                        .set_reg(RegisterName::SpsrAbt, cpsr.get_as_u32());
                    self.registers
                        .set_reg(RegisterName::Pc, ExceptionVector::DABT);
                }
                Exception::Reset => self
                    .registers
                    .set_reg(RegisterName::Pc, ExceptionVector::RESET),
            };

            // update the cpsr
            let cpsr: &mut Cpsr = self.registers.get_mut_cpsr();

            match exception {
                Exception::Swi => cpsr.set_operating_mode(OperatingMode::Svc),
                Exception::Udef => cpsr.set_operating_mode(OperatingMode::Und),
                Exception::Pabt | Exception::Dabt => cpsr.set_operating_mode(OperatingMode::Abt),
                Exception::Fiq => {
                    cpsr.set_operating_mode(OperatingMode::Fiq);
                    cpsr.set_interrupt_bit(Interruption::Fiq, BitState::Set);
                }
                Exception::Irq => cpsr.set_operating_mode(OperatingMode::Irq),
                Exception::Reset => {
                    cpsr.set_operating_mode(OperatingMode::Svc);
                    cpsr.set_interrupt_bit(Interruption::Fiq, BitState::Set);
                }
            };

            cpsr.set_interrupt_bit(Interruption::Irq, BitState::Set);
            cpsr.set_operating_state(OperatingState::Arm);
        }
    }
}
