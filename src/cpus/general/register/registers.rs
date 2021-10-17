use crate::cpus::general::register::{RegisterName, Cpsr};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Registers {
    // register from r0 to r7
    unbanked_registers: [u32; 13],

    /// The banked registers from r8 to r12
    fiq_registers: [u32; 5],

    r13: [u32; 6],
    r14: [u32; 6],
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
        }
    }

    pub fn get_mut_cpsr(&mut self) -> &mut Cpsr {
        &mut self.cpsr
    }

    pub fn get_ref_cpsr(&self) -> &Cpsr {
        &self.cpsr
    }
}
