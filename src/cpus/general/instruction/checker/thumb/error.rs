#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ThumbCheckerError {
    #[error("[Thumb Checker Error]: The instruction '{0:b}' \
            has an unknown instruction for its encoding group.\n\
            HINT: The encoding group is made by Bit[15:13]"
    )]
    UnknownInstructionOfGroup(u32),

    #[error("[Thumb Checker Error]: Unknown instruction group with the following instruction: '{0:b}'")]
    UnknownInstructionGroup(u32),

    #[error("[Thumb Checker Error]: Instruction '{0:b}' isn't supported yet. The instruction is probably added in a later architecture.")]
    SuccessorInstruction(u32),
}
