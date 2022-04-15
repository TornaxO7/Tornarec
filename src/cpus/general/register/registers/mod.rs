mod error;

pub use error::RegistersError;

use crate::{
    cpus::general::{
        register::{
            Cpsr,
            RegisterName,
        },
        OperatingMode,
    },
    ram::{
        data_types::DataType,
        Address,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Registers {
    // register from r0 to r12 (unbanked)
    unbanked_registers: [u32; 13],

    /// The banked registers from r8 to r12
    fiq_registers: [u32; 5],

    /// Stack pointers
    r13: [u32; 6],
    /// LR pointers
    r14: [u32; 6],

    /// PC
    r15: u32,

    cpsr: Cpsr,
    spsr: [Cpsr; 5],
}

impl Registers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_reg(&self, reg: RegisterName) -> u32 {
        match reg {
            RegisterName::R0 => self.unbanked_registers[0],
            RegisterName::R1 => self.unbanked_registers[1],
            RegisterName::R2 => self.unbanked_registers[2],
            RegisterName::R3 => self.unbanked_registers[3],
            RegisterName::R4 => self.unbanked_registers[4],
            RegisterName::R5 => self.unbanked_registers[5],
            RegisterName::R6 => self.unbanked_registers[6],
            RegisterName::R7 => self.unbanked_registers[7],
            RegisterName::R8 => self.unbanked_registers[8],
            RegisterName::R9 => self.unbanked_registers[9],
            RegisterName::R10 => self.unbanked_registers[10],
            RegisterName::R11 => self.unbanked_registers[11],
            RegisterName::R12 => self.unbanked_registers[12],

            // FIQ banked Registers
            RegisterName::R8Fiq => self.fiq_registers[0],
            RegisterName::R9Fiq => self.fiq_registers[1],
            RegisterName::R10Fiq => self.fiq_registers[2],
            RegisterName::R11Fiq => self.fiq_registers[3],
            RegisterName::R12Fiq => self.fiq_registers[4],

            RegisterName::R13 | RegisterName::Sp => self.r13[0],
            RegisterName::R13Svc | RegisterName::SpSvc => self.r13[1],
            RegisterName::R13Abt | RegisterName::SpAbt => self.r13[2],
            RegisterName::R13Und | RegisterName::SpUnd => self.r13[3],
            RegisterName::R13Irq | RegisterName::SpIrq => self.r13[4],
            RegisterName::R13Fiq | RegisterName::SpFiq => self.r13[5],

            RegisterName::R14 | RegisterName::Lr => self.r14[0],
            RegisterName::R14Svc | RegisterName::LrSvc => self.r14[1],
            RegisterName::R14Abt | RegisterName::LrAbt => self.r14[2],
            RegisterName::R14Und | RegisterName::LrUnd => self.r14[3],
            RegisterName::R14Irq | RegisterName::LrIrq => self.r14[4],
            RegisterName::R14Fiq | RegisterName::LrFiq => self.r14[5],

            RegisterName::R15 | RegisterName::Pc => self.r15,

            RegisterName::Cpsr => self.cpsr.get_as_u32(),

            RegisterName::SpsrSvc => self.spsr[0].get_as_u32(),
            RegisterName::SpsrAbt => self.spsr[1].get_as_u32(),
            RegisterName::SpsrUnd => self.spsr[2].get_as_u32(),
            RegisterName::SpsrIrq => self.spsr[3].get_as_u32(),
            RegisterName::SpsrFiq => self.spsr[4].get_as_u32(),
        }
    }

    pub fn set_reg(&mut self, reg: RegisterName, new_val: u32) {
        match reg {
            RegisterName::R0 => self.unbanked_registers[0] = new_val,
            RegisterName::R1 => self.unbanked_registers[1] = new_val,
            RegisterName::R2 => self.unbanked_registers[2] = new_val,
            RegisterName::R3 => self.unbanked_registers[3] = new_val,
            RegisterName::R4 => self.unbanked_registers[4] = new_val,
            RegisterName::R5 => self.unbanked_registers[5] = new_val,
            RegisterName::R6 => self.unbanked_registers[6] = new_val,
            RegisterName::R7 => self.unbanked_registers[7] = new_val,
            RegisterName::R8 => self.unbanked_registers[8] = new_val,
            RegisterName::R9 => self.unbanked_registers[9] = new_val,
            RegisterName::R10 => self.unbanked_registers[10] = new_val,
            RegisterName::R11 => self.unbanked_registers[11] = new_val,
            RegisterName::R12 => self.unbanked_registers[12] = new_val,

            // FIQ banked Registers
            RegisterName::R8Fiq => self.fiq_registers[0] = new_val,
            RegisterName::R9Fiq => self.fiq_registers[1] = new_val,
            RegisterName::R10Fiq => self.fiq_registers[2] = new_val,
            RegisterName::R11Fiq => self.fiq_registers[3] = new_val,
            RegisterName::R12Fiq => self.fiq_registers[4] = new_val,

            RegisterName::R13 | RegisterName::Sp => self.r13[0] = new_val,
            RegisterName::R13Svc | RegisterName::SpSvc => self.r13[1] = new_val,
            RegisterName::R13Abt | RegisterName::SpAbt => self.r13[2] = new_val,
            RegisterName::R13Und | RegisterName::SpUnd => self.r13[3] = new_val,
            RegisterName::R13Irq | RegisterName::SpIrq => self.r13[4] = new_val,
            RegisterName::R13Fiq | RegisterName::SpFiq => self.r13[5] = new_val,

            RegisterName::R14 | RegisterName::Lr => self.r14[0] = new_val,
            RegisterName::R14Svc | RegisterName::LrSvc => self.r14[1] = new_val,
            RegisterName::R14Abt | RegisterName::LrAbt => self.r14[2] = new_val,
            RegisterName::R14Und | RegisterName::LrUnd => self.r14[3] = new_val,
            RegisterName::R14Irq | RegisterName::LrIrq => self.r14[4] = new_val,
            RegisterName::R14Fiq | RegisterName::LrFiq => self.r14[5] = new_val,

            RegisterName::R15 | RegisterName::Pc => self.r15 = new_val,

            RegisterName::Cpsr => self.cpsr.set(new_val),

            RegisterName::SpsrSvc => self.spsr[0].set(new_val),
            RegisterName::SpsrAbt => self.spsr[1].set(new_val),
            RegisterName::SpsrUnd => self.spsr[2].set(new_val),
            RegisterName::SpsrIrq => self.spsr[3].set(new_val),
            RegisterName::SpsrFiq => self.spsr[4].set(new_val),
        };
    }

    pub fn get_spsr(&self, operating_mode: OperatingMode) -> Option<u32> {
        match operating_mode {
            OperatingMode::Svc => Some(self.get_reg(RegisterName::SpsrSvc)),
            OperatingMode::Abt => Some(self.get_reg(RegisterName::SpsrAbt)),
            OperatingMode::Und => Some(self.get_reg(RegisterName::SpsrUnd)),
            OperatingMode::Irq => Some(self.get_reg(RegisterName::SpsrIrq)),
            OperatingMode::Fiq => Some(self.get_reg(RegisterName::SpsrFiq)),
            _other => None,
        }
    }

    pub fn set_spsr(&mut self, new_val: u32) {
        let cpsr = self.get_ref_cpsr();
        match cpsr.get_operating_mode().unwrap() {
            OperatingMode::Svc => self.set_reg(RegisterName::SpsrSvc, new_val),
            OperatingMode::Abt => self.set_reg(RegisterName::SpsrAbt, new_val),
            OperatingMode::Und => self.set_reg(RegisterName::SpsrUnd, new_val),
            OperatingMode::Irq => self.set_reg(RegisterName::SpsrIrq, new_val),
            OperatingMode::Fiq => self.set_reg(RegisterName::SpsrFiq, new_val),
            other => unreachable!("{}", RegistersError::NoSpsr(other)),
        }
    }

    pub fn set_lr(&mut self, new_address: Address) {
        let cpsr = self.get_ref_cpsr();
        match cpsr.get_operating_mode().unwrap() {
            OperatingMode::Usr | OperatingMode::Sys => self.set_reg(RegisterName::Lr, new_address),
            OperatingMode::Fiq => self.set_reg(RegisterName::LrFiq, new_address),
            OperatingMode::Irq => self.set_reg(RegisterName::LrIrq, new_address),
            OperatingMode::Svc => self.set_reg(RegisterName::LrSvc, new_address),
            OperatingMode::Abt => self.set_reg(RegisterName::LrAbt, new_address),
            OperatingMode::Und => self.set_reg(RegisterName::LrUnd, new_address),
        }
    }

    pub fn get_adjusted_pc(&self) -> Address {
        Address::from(self.get_reg(RegisterName::Pc) - DataType::HALFWORD_SIZE)
    }

    pub fn move_pc_to_lr(&mut self) {
        let lr_val = {
            let pc_val = self.get_adjusted_pc();
            let lr_address = pc_val + DataType::HALFWORD_SIZE;
            lr_address
        };

        let cpsr = self.get_ref_cpsr();
        match cpsr.get_operating_mode().unwrap() {
            OperatingMode::Usr | OperatingMode::Sys => self.set_reg(RegisterName::Lr, lr_val),
            OperatingMode::Fiq => self.set_reg(RegisterName::LrFiq, lr_val),
            OperatingMode::Irq => self.set_reg(RegisterName::LrIrq, lr_val),
            OperatingMode::Svc => self.set_reg(RegisterName::LrSvc, lr_val),
            OperatingMode::Abt => self.set_reg(RegisterName::LrAbt, lr_val),
            OperatingMode::Und => self.set_reg(RegisterName::LrUnd, lr_val),
        }
    }

    pub fn move_current_spsr_to_cpsr(&mut self) {
        let cpsr = self.get_ref_cpsr();
        let operating_mode = cpsr.get_operating_mode().unwrap();
        match self.get_spsr(operating_mode) {
            Some(spsr) => self.set_reg(RegisterName::Cpsr, spsr),
            None => panic!("{}", RegistersError::NoSpsr(operating_mode)),
        }
    }

    pub fn get_mut_cpsr(&mut self) -> &mut Cpsr {
        &mut self.cpsr
    }

    pub fn get_ref_cpsr(&self) -> &Cpsr {
        &self.cpsr
    }
}
