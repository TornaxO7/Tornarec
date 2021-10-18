use crate:: cpus::general::{
    instruction::encodings::arm::DataProcessingImmediateShift,
    register::{Registers, types::ConditionBit},
    BitState,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ArmExecuter<'a>(&'a mut Registers);

impl<'a> ArmExecuter<'a> {

    pub fn new(registers: &'a mut Registers) -> Self {
        Self(registers)
    }

    pub fn data_processing_immediate_shift(&mut self, _data: DataProcessingImmediateShift) {
        // let cpsr = self.0.get_mut_cpsr();
        //
        // let (shifter_operand, shifter_carry_out) = if data.shift_imm == 0 {
        //     (data.rm, cpsr.get_condition_bit(ConditionBit::C).get_as_u32())
        // } else {
        //     (data.rm << data.shift_imm, 0, )
        // };
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
