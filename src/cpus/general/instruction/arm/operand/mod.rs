use crate::ram::Word;

use self::branch::BranchOperand;

use super::{
    opcode::ArmOpcode,
    types::Register,
    BitState,
};

mod branch;
mod data_processing;
mod load_store;
mod pld;
mod saturating;
mod semaphore;
mod multiply;
mod misc_arithmetic;
mod cpsr_access;
mod exception_generating;
mod coprocessor;

/// The operands are written as stated in the manual in page 109
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmOperand {
    Branch(BranchOperand),
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
        mode: coprocessor::LoadStoreCoprocessorMode,
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
    Multiply {
        rs: Register,
        rm: Register,
        mul_type: multiply::MultiplyType,
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
            ArmOpcode::B => branch::get_immed24(value),
            ArmOpcode::BL => branch::get_immed24(value),
            ArmOpcode::BIC => data_processing::get_operand(value),
            ArmOpcode::BKPT => exception_generating::get_bkpt(value),
            ArmOpcode::BLX1 => branch::get_link_exchange_immed(value),
            ArmOpcode::BLX2 => branch::get_register(value),
            ArmOpcode::BX => branch::get_register(value),
            ArmOpcode::CDP => coprocessor::get_cdp(value),
            ArmOpcode::CDP2 => coprocessor::get_cdp(value),
            ArmOpcode::CLZ => misc_arithmetic::get_clz(value),
            ArmOpcode::CMN => data_processing::get_operand(value),
            ArmOpcode::CMP => data_processing::get_operand(value),
            ArmOpcode::EOR => data_processing::get_operand(value),
            ArmOpcode::LDC => coprocessor::get_ldc_stc(value),
            ArmOpcode::LDC2 => coprocessor::get_ldc_stc(value),
            ArmOpcode::LDM => load_store::get_multiple(value),
            ArmOpcode::LDR => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::LDRB => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::LDRBT => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::LDRD => load_store::get_misc(value),
            ArmOpcode::LDRH => load_store::get_misc(value),
            ArmOpcode::LDRSB => load_store::get_misc(value),
            ArmOpcode::LDRSH => load_store::get_misc(value),
            ArmOpcode::LDRT => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::MCR => coprocessor::get_mcr_mrc(value),
            ArmOpcode::MCR2 => coprocessor::get_mcr_mrc(value),
            ArmOpcode::MCRR => coprocessor::get_mcrr_mrrc(value),
            ArmOpcode::MLA => multiply::get_normal_multiply(value),
            ArmOpcode::MOV => data_processing::get_operand(value),
            ArmOpcode::MRC => coprocessor::get_mcr_mrc(value),
            ArmOpcode::MRC2 => coprocessor::get_mcr_mrc(value),
            ArmOpcode::MRRC => coprocessor::get_mcrr_mrrc(value),
            ArmOpcode::MRS => cpsr_access::get_mrs(value),
            ArmOpcode::MSR => cpsr_access::get_msr(value),
            ArmOpcode::MUL => multiply::get_normal_multiply(value),
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
            ArmOpcode::SMLAXY => multiply::get_halfword(value),
            ArmOpcode::SMLAL => multiply::get_long(value),
            ArmOpcode::SMLALXY => multiply::get_halfword(value),
            ArmOpcode::SMLAWY => multiply::get_word_halfword(value),
            ArmOpcode::SMULXY => multiply::get_halfword(value),
            ArmOpcode::SMULL => multiply::get_long(value),
            ArmOpcode::SMULWY => multiply::get_word_halfword(value),
            ArmOpcode::STC => coprocessor::get_ldc_stc(value),
            ArmOpcode::STC2 => coprocessor::get_ldc_stc(value),
            ArmOpcode::STM => load_store::get_multiple(value),
            ArmOpcode::STR => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::STRB => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::STRBT => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::STRD => load_store::get_misc(value),
            ArmOpcode::STRH => load_store::get_misc(value),
            ArmOpcode::STRT => load_store::get_word_or_unsigned_byte(value),
            ArmOpcode::SUB => data_processing::get_operand(value),
            ArmOpcode::SWI => exception_generating::get_swi(value),
            ArmOpcode::SWP => semaphore::get_operand(value),
            ArmOpcode::SWPB => semaphore::get_operand(value),
            ArmOpcode::TEQ => data_processing::get_operand(value),
            ArmOpcode::TST => data_processing::get_operand(value),
            ArmOpcode::UMLAL => multiply::get_long(value),
            ArmOpcode::UMULL => multiply::get_long(value),
            _ => unreachable!(),
        }
    }
}
