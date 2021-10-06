use core::convert::From;

use crate::cpus::general::instruction::Instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShifterOperand {
    Immediate {
        rotate_imm: u8, 
        immed_8: u8,
    },
    ImmediateShift {
        shift_imm: u8,
        shift: u8,
        rm: u8,
    },
    RegisterShift {
        rs: u8,
        shift: u8,
        rm: u8,
    },
}

impl From<&Instruction> for ShifterOperand {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        if (instruction_val >> 25) & 0b1 == 1 {
            let rotate_imm = (instruction_val >> 8) & 0b1111;
            let immed_8 = instruction_val & 0b1111_1111;
            Self::Immediate {
                rotate_imm,
                immed_8,
            }
        } else if (instruction_val >> 4) & 0b1 == 0 {
            let shift_imm = (instruction_val >> 7) & 0b1111;
            let shift = (instruction_val >> 5) & 0b11;
            let rm = instruction_val & 0b1111;
            
            Self::ImmediateShift {
                shift_imm,
                shift,
                rm,
            }
        } else if (instruction_val >> 4) & 0b1 == 1 && (instruction_val >> 7) & 0b1 == 0 {
            let rs = (instruction_val >> 8) & 0b1111;
            let shift = (instruction_val >> 5) & 0b11;
            let rm = instruction_val & 0b1111;

            Self::RegisterShift {
                rs,
                shift,
                rm,
            }
        } else {
            unreachable!("[SHIFTER OPERAND ERROR]: Unknown operand: '{:b}'", instruction_val);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::ShifterOperand;
    use crate::cpus::general::instruction::Instruction;

    #[test]
    fn get_immediate() {
        let instruction = Instruction::from(0b0000_001_0000_0_0000_0000_1000_10000000);
        let expected_shifter_operand = ShifterOperand::Immediate {
            rotate_imm: 0b1000,
            immed_8: 0b1000_0000,
        };

        assert_eq!(ShifterOperand::from(&instruction), expected_shifter_operand);
    }

    #[test]
    fn get_immediate_shift() {
        let instruction = Instruction::from(0b0000_000_0000_0_0000_0000_1010_10_0_1010);
        let expected_shifter_operand = ShifterOperand::ImmediateShift {
            shift_imm: 0b1010,
            shift: 0b10,
            rm: 0b1010,
        };

        assert_eq!(ShifterOperand::from(&instruction), expected_shifter_operand);
    }

    #[test]
    fn get_register_shift() {
        let instruction = Instruction::from(0b0000_000_0000_0_0000_0000_1010_0_10_1_1010);
        let expected_shifter_operand = ShifterOperand::RegisterShift {
            rs: 0b1010,
            shift: 0b10,
            rm: 0b1010,
        };

        assert_eq!(ShifterOperand::from(&instruction), expected_shifter_operand);
    }
}
