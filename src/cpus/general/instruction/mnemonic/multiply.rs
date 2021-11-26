use crate::cpus::general::instruction::Instruction;

// The `S` suffix just means, that the `s_flag` is set, but this can be seen of the `Multiplies`
// struct instead of creating an extra name/flag in the enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiplyInstruction {
    /// Manual: Page 230
    MUL, 
    /// Manual: Page 216
    MLA, 
    /// Manual: Page 401
    UMULL, 
    /// Manual: Page 399
    UMLAL, 
    /// Manual: Page 318
    SMULL, 
    /// Manual: Page 296
    SMLAL,
    /// Manual: Page 397
    /// NOTE: Not supported on any CPU of the NintendoDS
    UMAAL,
}

impl From<&Instruction> for MultiplyInstruction {
    fn from(instruction: &Instruction) -> Self {
        match (instruction.val >> 21) & 0b111 {
            0b000 => Self::MUL,
            0b001 => Self::MLA,
            0b100 => Self::UMULL,
            0b101 => Self::UMLAL,
            0b110 => Self::SMULL,
            0b111 => Self::SMLAL,
            0b010 => Self::UMAAL,
            _ => unreachable!("[MultiplyInstruction Error]: Eeeh... what?"),
        }
    }
}
