use crate::ram::Word;

use self::data_processing::ShifterOperand;

use super::{opcode::ArmOpcode, Register, BitState};

mod branch;
mod data_processing;
mod breakpoint;

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
            CDP => ,
            CDP2 => ,
            CLZ => ,
            CMN => data_processing::get_operand(value),
            CMP => data_processing::get_operand(value),
            EOR => data_processing::get_operand(value),
            LDC => ,
            LDC2 => ,
            LDM => ,
            LDR => ,
            LDRB => ,
            LDRBT => ,
            LDRD => ,
            LDRH => ,
            LDRSB => ,
            LDRSH => ,
            LDRT => ,
            MCR => ,
            MCR2 => ,
            MCRR => ,
            MLA => ,
            MOV => data_processing::get_operand(value),
            MRC => ,
            MRC2 => ,
            MRRC => ,
            MRS => ,
            MSR => ,
            MUL => ,
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
            STC => ,
            STC2 => ,
            STM => ,
            STR => ,
            STRB => ,
            STRBT => ,
            STRD => ,
            STRH => ,
            STRT => ,
            SUB => data_processing::get_operand(value),
            SWI => get_swi(value),
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

fn get_swi(value: Word) -> ArmOperand {
    let immed24 = value >> 0b1111_1111_1111_1111_1111_1111;
    ArmOperand::SWI(immed24)
}

#[cfg(test)]
mod tests {
    use crate::cpus::general::instruction::arm::operand::{ArmOperand, get_swi};


    #[test]
    fn get_swi_operand() {
        let value = 0b0000_1111_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            ArmOperand::SWI(0b1111_1111_1111_1111_1111_1111),
            get_swi(value));
    }
}
