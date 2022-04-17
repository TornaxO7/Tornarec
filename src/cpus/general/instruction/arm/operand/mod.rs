use crate::ram::Word;

use self::{data_processing::ShifterOperand, load_store_coprocessor::LoadStoreCoprocessorMode, load_store_multiple::LoadStoreMultipleMode, load_store_word_byte::AddressingMode2, misc_load_store::AddressingMode3};

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
mod mla;
mod mrs;
mod msr;
mod mul;

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
    MLA {
        s: BitState,
        rd: Register,
        rn: Register,
        rs: Register,
        rm: Register,
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
    MUL {
        s: BitState,
        rd: Register,
        rs: Register,
        rm: Register,
    },
}

impl ArmOperand {
    pub fn get_operand(opcode: ArmOpcode, value: Word) -> Self {
        match opcode {
            ADC => data_processing::get_operand(value),
            ADD => data_processing::get_operand(value),
            AND => data_processing::get_operand(value),
            B => branch::normal(value),
            BL => branch::normal(value),
            BIC => data_processing::get_operand(value),
            BKPT => breakpoint::get_operand(value),
            BLX1 => branch::blx1(value),
            BLX2 => branch::register(value),
            BX => branch::register(value),
            CDP => cdp::get_operand(value),
            CDP2 => cdp::get_operand(value),
            CLZ => clz::get_operand(value),
            CMN => data_processing::get_operand(value),
            CMP => data_processing::get_operand(value),
            EOR => data_processing::get_operand(value),
            LDC => load_store_coprocessor::get_ldc_stc_operand(value),
            LDC2 => load_store_coprocessor::get_ldc_stc_operand(value),
            LDM => load_store_multiple::get_ldm_stm_operand(value),
            LDR => load_store_word_byte::get_operand(value),
            LDRB => load_store_word_byte::get_operand(value),
            LDRBT => load_store_word_byte::get_operand(value),
            LDRD => misc_load_store::get_operand(value),
            LDRH => misc_load_store::get_operand(value),
            LDRSB => misc_load_store::get_operand(value),
            LDRSH => misc_load_store::get_operand(value),
            LDRT => load_store_word_byte::get_operand(value),
            MCR => load_store_coprocessor::get_mcr_mrc_operand(value),
            MCR2 => load_store_coprocessor::get_mcr_mrc_operand(value),
            MCRR => load_store_coprocessor::get_mcrr_mrrc_operand(value),
            MLA => mla::get_operand(value),
            MOV => data_processing::get_operand(value),
            MRC => load_store_coprocessor::get_mcr_mrc_operand(value),
            MRC2 => load_store_coprocessor::get_mcr_mrc_operand(value),
            MRRC => load_store_coprocessor::get_mcrr_mrrc_operand(value),
            MRS => mrs::get_operand(value),
            MSR => msr::get_operand(value),
            MUL => mul::get_operand(value),
            MVN => data_processing::get_operand(value),
            ORR => data_processing::get_operand(value),
            PLD => ,
            QADD => ,
            QDADD => ,
            QDSUB => ,
            QSUB => ,
            RSB => data_processing::get_operand(value),
            RSC => data_processing::get_operand(value),
            SBC => data_processing::get_operand(value),
            SMLAXY => ,
            SMLAL => ,
            SMLALXY => ,
            SMLAWY => ,
            SMULXY => ,
            SMULL => ,
            SMULWY => ,
            STC => load_store_coprocessor::get_ldc_stc_operand(value),
            STC2 => load_store_coprocessor::get_ldc_stc_operand(value),
            STM => load_store_multiple::get_ldm_stm_operand(value),
            STR => ,
            STRB => ,
            STRBT => ,
            STRD => ,
            STRH => ,
            STRT => ,
            SUB => data_processing::get_operand(value),
            SWI => swi::get_operand(value),
            SWP => ,
            SWPB => ,
            TEQ => data_processing::get_operand(value),
            TST => data_processing::get_operand(value),
            UMLAL => ,
            UMULL => ,
            _ => unreachable!(),
        }
    }
}
