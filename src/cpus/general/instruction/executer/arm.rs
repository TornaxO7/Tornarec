use crate::{
    cpus::general::{
        instruction::encodings::arm::{
            DataProcessingImmediateShift,
            Miscellaneous1,
        },
        register::{ Cpsr, Spsr, GeneralRegister},
    },
    Ram,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArmExecuter<'a> {
    cpsr: &'a Cpsr,
    spsr: &'a Spsr,
    ram: &'a Ram,
    pc: &'a GeneralRegister,
}

impl<'a> ArmExecuter<'a> {
    pub fn new(cpsr: &'a Cpsr, spsr: &'a Spsr, ram: &'a Ram, pc: &'a GeneralRegister) -> Self {
        Self { cpsr, spsr, ram, pc}
    }

    pub fn data_processing_immediate_shift(&mut self, data: DataProcessingImmediateShift) {
        // let shifter_operand = {
        //     if data.shift_imm == 0 {
        //         self.cpsr.
        //     } else {
        //     }
        // }
    }

    pub fn miscellaneous_1(&mut self) {
    }

    pub fn data_processing_register_shift(&mut self) {
	}

    pub fn miscellaneous2(&mut self) {
	}

    pub fn multiplies(&mut self) {
	}

    pub fn extra_load_and_stores(&mut self) {
	}

    pub fn data_processing_immediate(&mut self) {
	}

    pub fn undefined_instruction(&mut self) {
	}

    pub fn move_immediate_to_status_register(&mut self) {
	}

    pub fn load_and_store_immediate_offset(&mut self) {
	}

    pub fn load_and_store_register_offset(&mut self) {
	}

    pub fn media_instructions(&mut self) {
	}

    pub fn architecturally_undefined(&mut self) {
	}

    pub fn load_and_store_multiple(&mut self) {
	}

    pub fn branch_and_branch_with_link(&mut self) {
	}

    pub fn coprocessor_load_and_store_and_double_register_transfers(&mut self) {
	}

    pub fn coprocessor_data_processing(&mut self) {
	}

    pub fn coprocessor_register_transfers(&mut self) {
	}

    pub fn software_interrupt(&mut self) {
	}

    pub fn unconditional_instructions(&mut self) {
	}

}
