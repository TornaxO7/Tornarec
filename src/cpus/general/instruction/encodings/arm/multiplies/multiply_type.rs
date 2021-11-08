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

#[cfg(test)]
mod tests {
    use super::{Instruction, MultiplyType, BitState};

    #[test]
    fn normal() {
        let instruction = Instruction {
            val: 0b0000_0000_0011_1110_1100_1000_1001_1111,
            .. Instruction::default()
        };
        
        let value = MultiplyType::from(&instruction);
        let expected_value = MultiplyType::Normal {
            a_flag: BitState::Set,
            s_flag: BitState::Set,
        };

        assert_eq!(expected_value, value, "{:#?} {:#?}", &expected_value, &value);
    }

    #[test]
    fn unsigned_multiply() {
        let instruction = Instruction {
            val: 0b0000_0000_0100_1111_1111_1111_1001_1111,
            .. Instruction::default()
        };

        let value = MultiplyType::from(&instruction);

        assert_eq!(MultiplyType::UnsignedMultiply, value);
    }

    #[test]
    fn long() {
        let instruction = Instruction {
            val: 0b0000_0000_1111_1110_1100_1000_1001_11111,
            .. Instruction::default()
        };

        let value = MultiplyType::from(&instruction);
        let expected_value = MultiplyType::Long {
            un_flag: BitState::Set,
            a_flag: BitState::Set,
            s_flag: BitState::Set,
        };

        assert_eq!(expected_value, value, "{:#?} {:#?}", &expected_value, &value);
    }
}
