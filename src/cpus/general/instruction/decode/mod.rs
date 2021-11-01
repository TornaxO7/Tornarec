pub mod arm;
mod decode_data;
mod thumb;

pub use decode_data::DecodeData;
pub use thumb::ThumbDecode;
