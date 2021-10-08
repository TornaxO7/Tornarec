use std::convert::From;

use crate::cpus::general::instruction::Instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Checker {
    Line1,
    Line2,
    Line3,
    Unknown,
}

impl From<&Instruction> for Checker {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        if (instruction_val >> 23) & 0b1_1111 == 0b0_0010
            && (instruction_val >> 20) & 0b1 == 0
            && (instruction_val >> 4) & 0b1 == 0
        {
            Self::Line1
        } else if (instruction_val >> 23) & 0b1_1111 == 0b0_0010
            && (instruction_val >> 20) & 0b1 == 0
            && (instruction_val >> 7) & 0b1 == 0
            && (instruction_val >> 4) & 0b1 == 1
        {
            Self::Line2
        } else if (instruction_val >> 23) & 0b1_1111 == 0b0_0110
            && (instruction_val >> 20) & 0b11 == 0b10
        {
            Self::Line3
        } else {
            Self::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Checker, Instruction};

    #[test]
    fn test_from_line1() {
        let instruction = Instruction::from(0b0000_00010_00_0_0000_0000_0000_000_0_0000);
        assert_eq!(Checker::from(&instruction), Checker::Line1);
    }

    #[test]
    fn test_from_line2() {
        let instruction = Instruction::from(0b0000_00010_00_0_0000_0000_0000_0_11_1_0000);
        assert_eq!(Checker::from(&instruction), Checker::Line2);
    }

    #[test]
    fn test_from_line3() {
        let instruction = Instruction::from(0b0000_00110_0_10_0000_0000_1111_1111_0000);
        assert_eq!(Checker::from(&instruction), Checker::Line3);
    }

    #[test]
    fn test_from_unknown() {
        let instruction = Instruction::from(0b101010101010101010101010101010);
        assert_eq!(Checker::from(&instruction), Checker::Unknown);
    }
}
