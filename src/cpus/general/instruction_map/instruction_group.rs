use crate::cpus::general::instruction::Instruction;

use core::convert::From;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum InstructionGroupError {
    #[error("[INSTRUCTION GROUP ERROR]: Miscellaneous instructions aren't implemented yet.")]
    MiscellaneousInstructions,

    #[error("[INSTRUCTION GROUP ERROR]: Multiplies and extra Load/Store instructions aren't implemented yet.")]
    MultipliesAndExtraLoadStore,

    #[error("[INSTRUCTION GROUP ERROR]: Move-immediate-to-status-register instruction isn't implemented yet")]
    MoveImmediateToCPSR,

    #[error("[INSTRUCTION GROUP ERROR]: Architecturally undefined")]
    ArchitecturallyUndefined,

    #[error("[INSTRUCTION GROUP ERROR]: Media instructions aren't implemented yet.")]
    MediaInstructions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionGroup {
    Branch,
    DataProcessing,
    Multiply,
    StatusRegisterAccess,
    LoadAndStore,
    LoadAndStoreMultiple,
    Semaphore,
    ExceptionGenerating,
    Coprocessor,

    UndefinedInstruction,
    SoftwareInterrupt,
}

impl From<&Instruction> for InstructionGroup {
    fn from(instruction: &Instruction) -> Self {
        let instruction_val = instruction.get_value_as_u32();

        match (instruction_val >> 25) & 0b111 {
            0b000 => {
                if ((instruction_val >> 23) & 0b11 == 0b10) && ((instruction_val >> 20) & 0b1 == 0)
                    && (((instruction_val >> 4) & 0b1 == 0) || ((instruction_val >> 7) & 0b1 == 0 && (instruction_val >> 4) & 0b1 == 1))
                {
                    todo!("{}", InstructionGroupError::MiscellaneousInstructions);
                } else if (instruction_val >> 7) & 0b1 == 1 && (instruction_val >> 4) & 0b1 == 1 {
                    todo!("{}", InstructionGroupError::MultipliesAndExtraLoadStore);
                } else {
                    Self::DataProcessing
                }
            },
            0b001 => {
                if (instruction_val >> 23) & 0b11 == 0b10 && (instruction_val >> 20) & 0b11 == 0b00 {
                    Self::UndefinedInstruction
                } else if (instruction_val >> 23) & 0b11 == 0b10 && (instruction_val >> 20) & 0b11 == 0b10 {
                    todo!("{}", InstructionGroupError::MoveImmediateToCPSR);
                } else {
                    Self::DataProcessing
                }
            },
            0b010 => Self::LoadAndStore,
            0b011 => {
                if (instruction_val >> 20) & 0b11111 == 0b11111 && (instruction_val >> 4) & 0b1111 == 0b1111 {
                    todo!("{}", InstructionGroupError::ArchitecturallyUndefined);
                } else if (instruction_val >> 4) & 0b1 == 1 {
                    todo!("{}", InstructionGroupError::MediaInstructions);
                } else {
                    Self::LoadAndStore
                }
            },
            0b100 => Self::LoadAndStoreMultiple,
            0b101 => Self::Branch,
            0b110 => Self::Coprocessor,
            0b111 => {
                if (instruction_val >> 24) & 0b1 == 1 {
                    Self::SoftwareInterrupt
                } else {
                    Self::Coprocessor
                }
            }
        }
    }
}
