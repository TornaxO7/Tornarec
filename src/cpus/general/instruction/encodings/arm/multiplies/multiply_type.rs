use crate::cpus::general::{BitState, Instruction};

/// Manual: Page 143
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiplyType {
    Normal {
        a_flag: BitState,
        s_flag: BitState,
    },
    UnsignedMultiply,
    Long {
        un_flag: BitState,
        a_flag: BitState,
        s_flag: BitState,
    },
}

impl From <&Instruction> for MultiplyType {
    fn from(instruction: &Instruction) -> Self {
        let a_flag = BitState::from(instruction.val >> 21);
        let s_flag = BitState::from(instruction.val >> 20);

        match (instruction.val >> 22) & 0b11 {
            0b00 => Self::Normal {
                a_flag,
                s_flag,
            },
            0b01 => Self::UnsignedMultiply,
            _ => {
                let un_flag = BitState::from(instruction.val >> 22);
                Self::Long {
                    un_flag,
                    a_flag,
                    s_flag,
                }
            }
        }
    }
}
