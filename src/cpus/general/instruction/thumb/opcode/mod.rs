use crate::{ram::Halfword, cpus::general::instruction::types::BitState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThumbOpcode {
    ADC,
    ADD,
    AND,
    ASR,
    B,
    BIC,
    BKPT,
    BL,
    BLX,
    BX,
    CMN,
    CMP,
    EOR,
    LDMIA,
    LDR,
    LDRB,
    LDRH,
    LDRSB,
    LDRSH,
    LSL,
    LSR,
    MOV,
    MUL,
    MVN,
    NEG,
    ORR,
    POP,
    PUSH,
    ROR,
    SBC,
    STMIA,
    STR,
    STRB,
    STRH,
    SUb,
    SWI,
    TST,
}

impl From<Halfword> for ThumbOpcode {
    fn from(value: Halfword) -> Self {
        let bit15_13 = (value >> 13) & 0b111;
        match bit15_13 {
            0b000 => {
                let bit12_11 = (value >> 11) & 0b11;
                if bit12_11 == 0b11 {
                    let bit10 = BitState::new(value.into(), 10);
                }
            },
            _ => todo!(),

            // 0b001 => ,
            // 0b010 => ,
            // 0b011 => ,
            // 0b100 => ,
            // 0b101 => ,
            // 0b110 => ,
            // 0b111 => ,
        };

        Self::TST
    }
}
