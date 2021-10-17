use crate::{
    cpus::general::{
        instruction::encodings::arm::DataProcessingImmediateShift,
        register::Registers,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub struct ArmExecuter<'a>(&'a mut Registers);

impl<'a> ArmExecuter<'a> {

    pub fn data_processing_immediate_shift(&self, _data: DataProcessingImmediateShift) {
    }

    pub fn miscellaneous_1(&self) {
    }

    pub fn data_processing_register_shift(&self) {
	}

    pub fn miscellaneous2(&self) {
	}

    pub fn multiplies(&self) {
	}

    pub fn extra_load_and_stores(&self) {
	}

    pub fn data_processing_immediate(&self) {
	}

    pub fn undefined_instruction(&self) {
	}

    pub fn move_immediate_to_status_register(&self) {
	}

    pub fn load_and_store_immediate_offset(&self) {
	}

    pub fn load_and_store_register_offset(&self) {
	}

    pub fn media_instructions(&self) {
	}

    pub fn architecturally_undefined(&self) {
	}

    pub fn load_and_store_multiple(&self) {
	}

    pub fn branch_and_branch_with_link(&self) {
	}

    pub fn coprocessor_load_and_store_and_double_register_transfers(&self) {
	}

    pub fn coprocessor_data_processing(&self) {
	}

    pub fn coprocessor_register_transfers(&self) {
	}

    pub fn software_interrupt(&self) {
	}

    pub fn unconditional_instructions(&self) {
	}

}
