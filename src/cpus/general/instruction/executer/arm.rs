use crate::{
    cpus::general::{
        instruction::encodings::{
            arm::DataProcessingImmediateShift,
            encoding_fields::DataProcessingInstruction,
        },
        register::{Registers, RegisterName, NormalizedRegister},
    },
    ram::{Ram, data_types::DataType},
};

#[derive(Debug, PartialEq, Eq)]
pub struct ArmExecuter<'a> {
    registers: &'a mut Registers,
    ram: &'a Ram,
}

impl<'a> ArmExecuter<'a> {

    pub fn new(registers: &'a mut Registers, ram: &'a Ram) -> Self {
        Self {
            registers,
            ram,
        }
    }

    pub fn get_next_instrution_val(&self) -> u32 {
        let pc = self.registers.get_reg(RegisterName::Pc);
        match DataType::get_word(&self.ram[pc + 8 .. pc + 12]) {
            Ok(word) => word.get_value_as_u32(),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn data_processing_immediate_shift(&mut self, data: DataProcessingImmediateShift) {
        let next_instruction_val = self.get_next_instrution_val();

        match data.opcode {
            DataProcessingInstruction::AND => {
            },
            DataProcessingInstruction::EOR => {
            },
            DataProcessingInstruction::SUB => {
			},
            DataProcessingInstruction::RSB => {
			},
            DataProcessingInstruction::ADD => {
			},
            DataProcessingInstruction::ADC => {
			},
            DataProcessingInstruction::SBC => {
			},
            DataProcessingInstruction::RSC => {
			},
            DataProcessingInstruction::TST => {
			},
            DataProcessingInstruction::TEQ => {
			},
            DataProcessingInstruction::CMP => {
			},
            DataProcessingInstruction::CMN => {
			},
            DataProcessingInstruction::ORR => {
			},
            DataProcessingInstruction::MOV => {
			},
            DataProcessingInstruction::BIC => {
			},
            DataProcessingInstruction::MVN => {
			},
        }
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
