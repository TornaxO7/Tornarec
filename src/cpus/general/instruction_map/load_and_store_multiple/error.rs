use super::LoadAndStoreMultiple;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum LoadAndStoreMultipleError {
    #[error("Unknown LDM instruction: {0:#?}")]
    UnknownLDMInstruction(LoadAndStoreMultiple),
    
    #[error("Unknown STM instruction: {0:#?}")]
    UnknownSTMInstruction(LoadAndStoreMultiple),
}
