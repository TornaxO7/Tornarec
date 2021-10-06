use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode3 {
    Immediate {
        immed_h: u8,
        s_flag: BitState,
        h_flag: BitState,
        immed_l: u8
    },
    Register {
        s_flag: BitState,
        h_flag: BitState,
        rm: u8,
    },
}

impl From<&Instruction> for AddressingMode3 {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        let immed_h = (instruction_val >> 8) & 0b1111;
        let sbz = immed_h;

        let s_flag = BitState::from(instruction_val >> 6);
        let h_flag = BitState::from(instruction_val >> 5);

        let immed_l = instruction_val & 0b1111;
        let rm = immed_l;

        if (instruction_val >> 22) & 0b1 == 1 {
            Self::Immediate {
                immed_h,
                s_flag,
                h_flag,
                immed_l,
            }
        } else {

            if sbz != 0b0000 {
                panic!("[ADDRESSING MODE 3 ERROR]: SBZ bit field in register encoding isn't zero!");
            }

            Self::Register {
                s_flag,
                h_flag,
                rm,
            }
        }
    }
}
