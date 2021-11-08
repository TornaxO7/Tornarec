use crate::cpus::general::instruction::{
    checker::arm::miscellaneous::MiscellaneousChecker,
    decode::DecodeData,
    encodings::arm::miscellaneous::{
        BranchAndLinkExchangeInstructionSetThumb,
        BranchExchangeInstructionSetJava,
        BranchExchangeInstructionSetThumb,
        CountLeadingZeros,
        MoveToStatusRegister,
        MoveStatusRegisterToRegister,
        SaturatingAddSubtract,
        SignedMultipliesType2,
        SoftwareBreakpoint,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Miscellaneous {
    MRS(MoveStatusRegisterToRegister),
    MSR(MoveToStatusRegister),
    BX(BranchExchangeInstructionSetThumb),
    BXJ(BranchExchangeInstructionSetJava),
    CLZ(CountLeadingZeros),
    BLX(BranchAndLinkExchangeInstructionSetThumb),
    QADDOrQSUB(SaturatingAddSubtract),
    BKPT(SoftwareBreakpoint),
    /// This includes:
    ///     - SMLA<x><y>
    ///     - SMLAW<y>
    ///     - SMULW<y>
    ///     - SMLAL<x><y>
    ///     - SMUL<x><y>
    SignedMultipliesType2(SignedMultipliesType2),
    Unknown,
}

impl<'a> From<DecodeData<'a>> for Miscellaneous {
    fn from(data: DecodeData<'a>) -> Self {
        match MiscellaneousChecker::from(&data.instruction) {
            MiscellaneousChecker::MoveToStatusRegister => Self::MSR(MoveToStatusRegister::from(data)),
            MiscellaneousChecker::MoveStatusRegisterToRegister => Self::MRS(MoveStatusRegisterToRegister::from(data)),
            MiscellaneousChecker::BranchExchangeInstructionSetThumb => {
                Self::BX(BranchExchangeInstructionSetThumb::from(
                    data,
                ))
            }
            MiscellaneousChecker::BranchExchangeInstructionSetJava => {
                Self::BXJ(BranchExchangeInstructionSetJava::from(data))
            }
            MiscellaneousChecker::CountLeadingZeros => {
                Self::CLZ(CountLeadingZeros::from(data))
            }
            MiscellaneousChecker::BranchAndLinkExchangeInstructionSetThumb => {
                Self::BLX(
                    BranchAndLinkExchangeInstructionSetThumb::from(data),
                )
            }
            MiscellaneousChecker::SaturatingAddSubtract => {
                Self::QADDOrQSUB(SaturatingAddSubtract::from(data))
            }
            MiscellaneousChecker::SoftwareBreakpoint => {
                Self::BKPT(SoftwareBreakpoint::from(data))
            }
            MiscellaneousChecker::SignedMultiplies => {
                Self::SignedMultipliesType2(SignedMultipliesType2::from(data))
            }
            MiscellaneousChecker::Unknown => Self::Unknown,
        }
    }
}
