use crate::ram::Word;

use super::{
    opcode::ArmOpcode,
    types::Register,
    BitState,
};

mod branch;
mod breakpoint;
mod cdp;
mod clz;
mod data_processing;
mod halfword_multiply;
mod load_store;
mod load_store_coprocessor;
mod long_multiply;
mod mrs;
mod msr;
mod normal_multiply;
mod pld;
mod saturating;
mod semaphore;
mod swi;
mod word_halfword_multiply;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmOperand {
    Branch(u32),
    BLX1 {
        h: BitState,
        immed24: u32,
    },
    BRegister(Register),
    DataProcessing {
        s: BitState,
        rn: Register,
        rd: Register,
        shifter_operand: data_processing::ShifterOperand,
    },
    BKPT {
        immed1: u16,
        immed2: u8,
    },
    SWI(u32),
    CDP {
        opcode1: u8,
        crn: Register,
        crd: Register,
        num: u8,
        opcode2: u8,
        crm: Register,
    },
    CLZ {
        rd: Register,
        rm: Register,
    },
    LDCandSTC {
        u: BitState,
        n: BitState,
        rn: Register,
        crd: Register,
        cp_num: u8,
        immed8: u8,
        mode: load_store_coprocessor::LoadStoreCoprocessorMode,
    },
    MCRandMRC {
        opcode1: u8,
        crn: Register,
        rd: Register,
        cp_num: u8,
        opcode2: u8,
        crm: Register,
    },
    MCRRandMRRC {
        rn: Register,
        rd: Register,
        cp_num: u8,
        opcode: u8,
        crm: Register,
    },
    LoadStore {
        rn: Register,
        load_store_type: load_store::LoadStoreType,
    },
    NormalMultiply {
        s: BitState,
        rd: Register,
        rs: Register,
        rm: Register,
        mul_type: normal_multiply::NormalMultiplyType,
    },
    MRS {
        r: BitState,
        rd: Register,
    },
    MSR {
        r: BitState,
        // Note: Probably using something similar like RegisterList
        field_mask: u8,
        shifter_operand: data_processing::ShifterOperand,
    },
    PLD {
        u: BitState,
        rn: Register,
        addr_mode: load_store::AddressingMode2,
    },
    Saturating {
        rn: Register,
        rd: Register,
        rm: Register,
    },
    HalfwordMultiply {
        rs: Register,
        y: BitState,
        x: BitState,
        rm: Register,
        mul_type: halfword_multiply::HalfwordMultiplyType,
    },
    LongMultiply {
        s: BitState,
        rdhi: u8,
        rdlo: u8,
        rs: Register,
        rm: Register,
    },
    WordHalfwordMultiply {
        rd: Register,
        rs: Register,
        y: BitState,
        rm: Register,
        mul_type: word_halfword_multiply::WordHalfwordMultiplyType,
    },
    Semaphore {
        rn: Register,
        rd: Register,
        rm: Register,
    },
}

impl ArmOperand {
    pub fn get_operand(opcode: ArmOpcode, value: Word) -> Self {
        match opcode {
            ArmOpcode::ADC => data_processing::get_operand(value),
            ArmOpcode::ADD => data_processing::get_operand(value),
            ArmOpcode::AND => data_processing::get_operand(value),
            ArmOpcode::B => branch::get_immed24_operand(value),
            ArmOpcode::BL => branch::get_immed24_operand(value),
            ArmOpcode::BIC => data_processing::get_operand(value),
            ArmOpcode::BKPT => breakpoint::get_operand(value),
            ArmOpcode::BLX1 => branch::get_blx1_operand(value),
            ArmOpcode::BLX2 => branch::get_register_operand(value),
            ArmOpcode::BX => branch::get_register_operand(value),
            ArmOpcode::CDP => cdp::get_operand(value),
            ArmOpcode::CDP2 => cdp::get_operand(value),
            ArmOpcode::CLZ => clz::get_operand(value),
            ArmOpcode::CMN => data_processing::get_operand(value),
            ArmOpcode::CMP => data_processing::get_operand(value),
            ArmOpcode::EOR => data_processing::get_operand(value),
            ArmOpcode::LDC => load_store_coprocessor::get_ldc_stc_operand(value),
            ArmOpcode::LDC2 => load_store_coprocessor::get_ldc_stc_operand(value),
            ArmOpcode::LDM => load_store::get_multiple(value),
            ArmOpcode::LDR => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::LDRB => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::LDRBT => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::LDRD => load_store::get_misc(value),
            ArmOpcode::LDRH => load_store::get_misc(value),
            ArmOpcode::LDRSB => load_store::get_misc(value),
            ArmOpcode::LDRSH => load_store::get_misc(value),
            ArmOpcode::LDRT => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::MCR => load_store_coprocessor::get_mcr_mrc_operand(value),
            ArmOpcode::MCR2 => load_store_coprocessor::get_mcr_mrc_operand(value),
            ArmOpcode::MCRR => load_store_coprocessor::get_mcrr_mrrc_operand(value),
            ArmOpcode::MLA => normal_multiply::get_mla_operand(value),
            ArmOpcode::MOV => data_processing::get_operand(value),
            ArmOpcode::MRC => load_store_coprocessor::get_mcr_mrc_operand(value),
            ArmOpcode::MRC2 => load_store_coprocessor::get_mcr_mrc_operand(value),
            ArmOpcode::MRRC => load_store_coprocessor::get_mcrr_mrrc_operand(value),
            ArmOpcode::MRS => mrs::get_operand(value),
            ArmOpcode::MSR => msr::get_operand(value),
            ArmOpcode::MUL => normal_multiply::get_mul_operand(value),
            ArmOpcode::MVN => data_processing::get_operand(value),
            ArmOpcode::ORR => data_processing::get_operand(value),
            ArmOpcode::PLD => pld::get_operand(value),
            ArmOpcode::QADD => saturating::get_operand(value),
            ArmOpcode::QDADD => saturating::get_operand(value),
            ArmOpcode::QDSUB => saturating::get_operand(value),
            ArmOpcode::QSUB => saturating::get_operand(value),
            ArmOpcode::RSB => data_processing::get_operand(value),
            ArmOpcode::RSC => data_processing::get_operand(value),
            ArmOpcode::SBC => data_processing::get_operand(value),
            ArmOpcode::SMLAXY => halfword_multiply::get_operand(value),
            ArmOpcode::SMLAL => long_multiply::get_operand(value),
            ArmOpcode::SMLALXY => halfword_multiply::get_operand(value),
            ArmOpcode::SMLAWY => word_halfword_multiply::get_operand(value),
            ArmOpcode::SMULXY => halfword_multiply::get_operand(value),
            ArmOpcode::SMULL => long_multiply::get_operand(value),
            ArmOpcode::SMULWY => word_halfword_multiply::get_operand(value),
            ArmOpcode::STC => load_store_coprocessor::get_ldc_stc_operand(value),
            ArmOpcode::STC2 => load_store_coprocessor::get_ldc_stc_operand(value),
            ArmOpcode::STM => load_store::get_multiple(value),
            ArmOpcode::STR => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::STRB => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::STRBT => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::STRD => load_store::get_misc(value),
            ArmOpcode::STRH => load_store::get_misc(value),
            ArmOpcode::STRT => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::SUB => data_processing::get_operand(value),
            ArmOpcode::SWI => swi::get_operand(value),
            ArmOpcode::SWP => semaphore::get_operand(value),
            ArmOpcode::SWPB => semaphore::get_operand(value),
            ArmOpcode::TEQ => data_processing::get_operand(value),
            ArmOpcode::TST => data_processing::get_operand(value),
            ArmOpcode::UMLAL => long_multiply::get_operand(value),
            ArmOpcode::UMULL => long_multiply::get_operand(value),
            _ => unreachable!(),
        }
    }
}
