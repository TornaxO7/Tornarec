use crate::cpus::general::{
    bit_state::BitState,
    instruction::Instruction,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchOperand {
    BOrBL {
        l: BitState,
        signed_immed_24: i32
    },
    Bx {
        rm: u8,
        switch_to_thumb: BitState,
    },
}

impl BranchOperand {
    pub fn is_b_or_bl_instruction(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();

        (instruction_val >> 25) & 0b111 == 0b101
    }

    pub fn is_bx_instruction(instruction: &Instruction) -> bool {
        let instruction_val = instruction.get_value_as_u32();

        (instruction_val >> 20) & 0b1111_1111 == 0b0001_0010
            && (instruction_val >> 4) & 0b1111 == 0b0001
    }
}

#[cfg(test)]
mod tests {
    
    use super::BranchOperand;
    use crate::cpus::general::instruction::Instruction;

    #[test]
    fn test_is_b_or_bl_instruction() {
        let b_instruction = Instruction::from(0b0000_1010_0000_0000_0000_0000_0000_0000);
        let bl_instruction = Instruction::from(0b0000_1011_0000_0000_0000_0000_0000_0000);

        let invalid_instruction = Instruction::from(0b101010);

        assert!(BranchOperand::is_b_or_bl_instruction(&b_instruction));
        assert!(BranchOperand::is_b_or_bl_instruction(&bl_instruction));
        assert!(!BranchOperand::is_bx_instruction(&invalid_instruction));
    }

    #[test]
    fn test_is_bx_instruction() {
        let bx_instruction = Instruction::from(0b0000_0001_0010_1111_1111_1111_0001_0000);
        assert!(BranchOperand::is_bx_instruction(&bx_instruction));
    }
}
