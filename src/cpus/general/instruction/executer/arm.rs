use crate::{cpus::general::{BitState, instruction::encodings::{arm::DataProcessingImmediateShift, encoding_fields::{DataProcessingInstruction, RegisterOrValue}}, register::{
            types::ConditionBit,
            NormalizedRegister,
            RegisterName,
            Registers,
        }}, ram::{
        data_types::DataType,
        Ram,
    }};

#[derive(Debug, PartialEq, Eq)]
pub struct ArmExecuter<'a> {
    registers: &'a mut Registers,
    ram: &'a Ram,
}

impl<'a> ArmExecuter<'a> {
    pub fn new(registers: &'a mut Registers, ram: &'a Ram) -> Self {
        Self { registers, ram }
    }

    pub fn data_processing_immediate_shift(&mut self, data: DataProcessingImmediateShift) {
        match data.opcode {
            DataProcessingInstruction::AND => {}
            DataProcessingInstruction::EOR => {}
            DataProcessingInstruction::SUB => {}
            DataProcessingInstruction::RSB => {}
            DataProcessingInstruction::ADD => {}
            DataProcessingInstruction::ADC => {
                let cpsr = self.registers.get_ref_cpsr();
                let c_flag = cpsr.get_condition_bit(ConditionBit::C);

                let rn: u32 = match data.rn {
                    RegisterOrValue::Register(reg) => self.registers.get_reg(reg.get_reg()),
                    RegisterOrValue::Value(val) => val,
                };

                let rd = rn + data.shifter_operand.shifter_operand + c_flag.get_as_u32();

                if data.s_flag.is_set() {
                    if NormalizedRegister::from(rd) == RegisterName::Pc {
                        if cpsr.current_mode_has_spsr() {
                            // TODO: The execution stuff
                            // let cpsr = self.registers.get_mut_cpsr();
                            // cpsr.set();
                        } else {
                            panic!("Houston, we've got a problem...");
                        }
                    } else {
                        let carry = rn + data.shifter_operand.shifter_operand + c_flag.get_as_u32();
                        let overflow: Option<u32> = rn 
                            .checked_add(data.shifter_operand.shifter_operand)
                            .and_then(|rn| rn.checked_add(c_flag.get_as_u32()));

                        let cpsr = self.registers.get_mut_cpsr();
                        cpsr.set_condition_bit(ConditionBit::N, BitState::from(rd >> 31));
                        cpsr.set_condition_bit(ConditionBit::Z, BitState::from(rd == 0));
                        cpsr.set_condition_bit(ConditionBit::C, BitState::from(carry >> 31));
                        cpsr.set_condition_bit(ConditionBit::V, BitState::from(overflow.is_none()));
                    }
                }

            }
            DataProcessingInstruction::SBC => {}
            DataProcessingInstruction::RSC => {}
            DataProcessingInstruction::TST => {}
            DataProcessingInstruction::TEQ => {}
            DataProcessingInstruction::CMP => {}
            DataProcessingInstruction::CMN => {}
            DataProcessingInstruction::ORR => {}
            DataProcessingInstruction::MOV => {}
            DataProcessingInstruction::BIC => {}
            DataProcessingInstruction::MVN => {}
        }
    }

    pub fn miscellaneous_1(&self) {}

    pub fn data_processing_register_shift(&self) {}

    pub fn miscellaneous2(&self) {}

    pub fn multiplies(&self) {}

    pub fn extra_load_and_stores(&self) {}

    pub fn data_processing_immediate(&self) {}

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
