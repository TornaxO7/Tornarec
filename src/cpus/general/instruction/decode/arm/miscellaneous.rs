use crate::cpus::general::instruction::{
    checker::arm::miscellaneous::MiscellaneousChecker,
    decode::DecodeData,
    encodings::arm::miscellaneous::{
        BranchAndLinkExchangeInstructionSetThumb,
        BranchExchangeInstructionSetJava,
        BranchExchangeInstructionSetThumb,
        CountLeadingZeros,
        MoveImmediateToStatusRegister,
        MoveRegisterToStatusRegister,
        MoveStatusRegisterToRegister,
        SaturatingAddSubtract,
        SignedMultipliesType2,
        SoftwareBreakpoint,
    },
};

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
    Unknown,
}

impl<'a> From<DecodeData<'a>> for Miscellaneous {
    fn from(data: DecodeData<'a>) -> Self {
        match MiscellaneousChecker::from(&data.instruction) {
            MiscellaneousChecker::MoveStatusRegisterToRegister => {
                Self::MoveStatusRegisterToRegister(MoveStatusRegisterToRegister::from(data))
            }
            MiscellaneousChecker::MoveRegisterToStatusRegister => {
                Self::MoveRegisterToStatusRegister(MoveRegisterToStatusRegister::from(data))
            }
            MiscellaneousChecker::MoveImmediateToStatusRegister => {
                Self::MoveImmediateToStatusRegister(MoveImmediateToStatusRegister::from(data))
            }
            MiscellaneousChecker::BranchExchangeInstructionSetThumb => {
                Self::BranchExchangeInstructionSetThumb(BranchExchangeInstructionSetThumb::from(
                    data,
                ))
            }
            MiscellaneousChecker::BranchExchangeInstructionSetJava => {
                Self::BranchExchangeInstructionSetJava(BranchExchangeInstructionSetJava::from(data))
            }
            MiscellaneousChecker::CountLeadingZeros => {
                Self::CountLeadingZeros(CountLeadingZeros::from(data))
            }
            MiscellaneousChecker::BranchAndLinkExchangeInstructionSetThumb => {
                Self::BranchAndLinkExchangeInstructionSetThumb(
                    BranchAndLinkExchangeInstructionSetThumb::from(data),
                )
            }
            MiscellaneousChecker::SaturatingAddSubtract => {
                Self::SaturatingAddSubtract(SaturatingAddSubtract::from(data))
            }
            MiscellaneousChecker::SoftwareBreakpoint => {
                Self::SoftwareBreakpoint(SoftwareBreakpoint::from(data))
            }
            MiscellaneousChecker::SignedMultiplies => {
                Self::SignedMultipliesType2(SignedMultipliesType2::from(data))
            }
            MiscellaneousChecker::Unknown => Self::Unknown,
        }
    }
}
