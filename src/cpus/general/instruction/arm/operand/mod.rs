use crate::ram::Word;

use self::{data_processing::ShifterOperand, load_store_coprocessor::LoadStoreCoprocessorMode, load_store_multiple::LoadStoreMultipleMode, load_store_word_byte::AddressingMode2, misc_load_store::AddressingMode3, normal_multiply::NormalMultiplyType, halfword_multiply::HalfwordMultiplyType, word_halfword_multiply::WordHalfwordMultiplyType};

use super::{types::{Register, RegisterList}, BitState, opcode::ArmOpcode};

mod branch;
mod data_processing;
mod breakpoint;
mod swi;
mod cdp;
mod clz;
mod load_store_coprocessor;
mod load_store_multiple;
mod load_store_word_byte;
mod misc_load_store;
mod mrs;
mod msr;
mod normal_multiply;
mod pld;
mod saturating;
mod halfword_multiply;
mod long_multiply;
mod word_halfword_multiply;
mod semaphore;

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
        shifter_operand: ShifterOperand,
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
        mode: LoadStoreCoprocessorMode,
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
    LDMandSTM {
        s: BitState,
        w: BitState,
        rn: Register,
        register_list: RegisterList,
        mode: LoadStoreMultipleMode,
    },
    LoadStoreWordOrByte {
        p: BitState,
        u: BitState,
        b: BitState,
        w: BitState,
        rn: Register,
        rd: Register,
        offset: AddressingMode2,
    },
    MiscLoadStore {
        p: BitState,
        u: BitState,
        w: BitState,
        rn: Register,
        rd: Register,
        s: BitState,
        h: BitState,
        offset: AddressingMode3,
    },
    NormalMultiply {
        s: BitState,
        rd: Register,
        rs: Register,
        rm: Register,
        mul_type: NormalMultiplyType,
    },
    MRS {
        r: BitState,
        rd: Register,
    },
    MSR {
        r: BitState,
        // Note: Probably using something similar like RegisterList
        field_mask: u8,
        shifter_operand: ShifterOperand,
    },
    PLD {
        u: BitState,
        rn: Register,
        addr_mode: AddressingMode2,
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
        mul_type: HalfwordMultiplyType,
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
        mul_type: WordHalfwordMultiplyType,
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
            ArmOpcode::B => branch::normal(value),
            ArmOpcode::BL => branch::normal(value),
            ArmOpcode::BIC => data_processing::get_operand(value),
            ArmOpcode::BKPT => breakpoint::get_operand(value),
            ArmOpcode::BLX1 => branch::blx1(value),
            ArmOpcode::BLX2 => branch::register(value),
            ArmOpcode::BX => branch::register(value),
            ArmOpcode::CDP => cdp::get_operand(value),
            ArmOpcode::CDP2 => cdp::get_operand(value),
            ArmOpcode::CLZ => clz::get_operand(value),
            ArmOpcode::CMN => data_processing::get_operand(value),
            ArmOpcode::CMP => data_processing::get_operand(value),
            ArmOpcode::EOR => data_processing::get_operand(value),
            ArmOpcode::LDC => load_store_coprocessor::get_ldc_stc_operand(value),
            ArmOpcode::LDC2 => load_store_coprocessor::get_ldc_stc_operand(value),
            ArmOpcode::LDM => load_store_multiple::get_ldm_stm_operand(value),
            ArmOpcode::LDR => load_store_word_byte::get_operand(value),
            ArmOpcode::LDRB => load_store_word_byte::get_operand(value),
            ArmOpcode::LDRBT => load_store_word_byte::get_operand(value),
            ArmOpcode::LDRD => misc_load_store::get_operand(value),
            ArmOpcode::LDRH => misc_load_store::get_operand(value),
            ArmOpcode::LDRSB => misc_load_store::get_operand(value),
            ArmOpcode::LDRSH => misc_load_store::get_operand(value),
            ArmOpcode::LDRT => load_store_word_byte::get_operand(value),
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
            ArmOpcode::STM => load_store_multiple::get_ldm_stm_operand(value),
            ArmOpcode::STR => load_store_word_byte::get_operand(value),
            ArmOpcode::STRB => load_store_word_byte::get_operand(value),
            ArmOpcode::STRBT => load_store_word_byte::get_operand(value),
            ArmOpcode::STRD => misc_load_store::get_operand(value),
            ArmOpcode::STRH => misc_load_store::get_operand(value),
            ArmOpcode::STRT => load_store_word_byte::get_operand(value),
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
