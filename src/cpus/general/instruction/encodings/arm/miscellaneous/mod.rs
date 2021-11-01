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

use move_status_register_to_register::MoveStatusRegisterToRegister;
use move_register_to_status_register::MoveRegisterToStatusRegister;
use move_immediate_to_status_register::MoveImmediateToStatusRegister;
use branch_exchange_instruction_set_thumb::BranchExchangeInstructionSetThumb;
use branch_exchange_instruction_set_java::BranchExchangeInstructionSetJava;
use count_leading_zeros::CountLeadingZeros;
use branch_and_link_exchange_instruction_set_thumb::BranchAndLinkExchangeInstructionSetThumb;
use saturating_add_subtract::SaturatingAddSubtract;
use software_breakpoint::SoftwareBreakpoint;
use signed_multiplies_type_2::SignedMultipliesType2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Miscellaneous {
    MoveStatusRegisterToRegister(MoveStatusRegisterToRegister),
    MoveRegisterToStatusRegister(MoveRegisterToStatusRegister),
    MoveImmediateToStatusRegister(MoveImmediateToStatusRegister),
    BranchExchangeInstructionSetThumb(BranchExchangeInstructionSetThumb),
    BranchExchangeInstructionSetJava(BranchExchangeInstructionSetJava),
    CountLeadingZeros(CountLeadingZeros),
    BranchAndLinkExchangeInstructionSetThumb(BranchAndLinkExchangeInstructionSetThumb),
    SaturatingAddSubtract(SaturatingAddSubtract),
    SoftwareBreakpoint(SoftwareBreakpoint),
    SignedMultipliesType2(SignedMultipliesType2),
}
