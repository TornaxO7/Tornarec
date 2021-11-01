mod error;
mod move_status_register_to_register;
mod move_register_to_status_register;
mod move_immediate_to_status_register;
mod branch_exchange_instruction_set_thumb;
mod branch_exchange_instruction_set_java;
mod count_leading_zeros;
mod branch_and_link_exchange_instruction_set_thumb;
mod saturating_add_subtract;
mod software_breakpoint;
mod signed_multiplies_type_2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Miscellaneous {
    MoveStatusRegisterToRegister,
    MoveRegisterToStatusRegister,
    MoveImmediateToStatusRegister,
    BranchExchangeInstructionSetThumb,
    BranchExchangeInstructionSetJava,
    CountLeadingZeros,
    BranchAndLinkExchangeInstructionSetThumb,
    SaturatingAddSubtract,
    SoftwareBreakpoint,
    SignedMultipliesType2,
}
