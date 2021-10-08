#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Miscellaneous1Error {
    #[error("[Miscellaneous1 Error]: Unknown Miscellaneous1 instruction: '{0:b}'.")]
    UnknownMiscellaneous(u32),
}
